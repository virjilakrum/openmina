use std::collections::BTreeMap;
use std::ffi::{OsStr, OsString};
use std::io;
use std::mem::size_of;
use std::process::Stdio;
use std::sync::Arc;
use std::time::Duration;

use binprot::{BinProtRead, BinProtWrite};
use mina_p2p_messages::v2::{
    CurrencyFeeStableV1, NonZeroCurvePoint, SnarkWorkerWorkerRpcsVersionedGetWorkV2TResponse,
    SnarkWorkerWorkerRpcsVersionedGetWorkV2TResponseA0,
    SnarkWorkerWorkerRpcsVersionedSubmitWorkV2TQuery,
    UnsignedExtendedUInt64Int64ForVersionTagsStableV1,
};
use rand::prelude::*;
use serde::Serialize;
use shared::log::inner::Level;
use snarker::external_snark_worker::{
    ExternalSnarkWorkerError, ExternalSnarkWorkerEvent, ExternalSnarkWorkerService,
    ExternalSnarkWorkerStartAction, ExternalSnarkWorkerWorkError, SnarkWorkResult, SnarkWorkSpec,
};
use snarker::p2p::service_impl::TaskSpawner;
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};
use tokio::process::Command;
use tokio::select;
use tokio::sync::{mpsc, oneshot};

use snarker::account::AccountPublicKey;
use snarker::event_source::{
    Event, EventSourceProcessEventsAction, EventSourceWaitForEventsAction,
    EventSourceWaitTimeoutAction,
};
use snarker::ledger::LedgerCtx;
use snarker::p2p::channels::ChannelId;
use snarker::p2p::connection::outgoing::P2pConnectionOutgoingInitOpts;
use snarker::p2p::identity::SecretKey;
use snarker::p2p::service_impl::libp2p::Libp2pService;
use snarker::p2p::service_impl::webrtc_rs::{Cmd, P2pServiceCtx, P2pServiceWebrtcRs, PeerState};
use snarker::p2p::service_impl::webrtc_rs_with_libp2p::{self, P2pServiceWebrtcRsWithLibp2p};
use snarker::p2p::{P2pConfig, P2pEvent, PeerId};
use snarker::rpc::RpcRequest;
use snarker::service::EventSourceService;
use snarker::snark::block_verify::{
    SnarkBlockVerifyError, SnarkBlockVerifyId, SnarkBlockVerifyService, VerifiableBlockWithHash,
};
use snarker::snark::{SnarkEvent, VerifierIndex, VerifierKind, VerifierSRS};
use snarker::snark_pool::SnarkPoolConfig;
use snarker::stats::Stats;
use snarker::{Config, LedgerConfig, SnarkConfig, SnarkerConfig, State, TransitionFrontierConfig};

mod http_server;

mod rpc;
use rpc::RpcP2pConnectionOutgoingResponse;
mod tracing;

/// Openmina snarker
#[derive(Debug, clap::Args)]
pub struct Snarker {
    /// Chain ID
    #[arg(long, short = 'i', env)]
    pub chain_id: String,

    /// Snarker public key
    #[arg(long, short = 'k', env)]
    pub public_key: AccountPublicKey,

    /// Snark fee, in Mina
    #[arg(long, short, env, default_value_t = 1_000_000)]
    pub fee: u64,

    /// Automatically commit to snark jobs
    #[arg(long, short = 'a', env, default_value_t = true)]
    pub auto_commit: bool,

    /// Port to listen to
    #[arg(long, short, default_value = "3000")]
    pub port: u16,

    /// Verbosity level
    #[arg(long, short, default_value = "info")]
    pub verbosity: Level,

    #[arg(long = "peer", default_values_t = default_peers())]
    pub peers: Vec<P2pConnectionOutgoingInitOpts>,

    /// Mina snark worker path
    #[arg(long, short = 'e', env)]
    pub mina_exe_path: Option<OsString>,
}

fn default_peers() -> Vec<P2pConnectionOutgoingInitOpts> {
    [
        "/dns4/seed-1.berkeley.o1test.net/tcp/10000/p2p/12D3KooWAdgYL6hv18M3iDBdaK1dRygPivSfAfBNDzie6YqydVbs",
        "/dns4/seed-2.berkeley.o1test.net/tcp/10001/p2p/12D3KooWLjs54xHzVmMmGYb7W5RVibqbwD1co7M2ZMfPgPm7iAag",
        "/dns4/seed-3.berkeley.o1test.net/tcp/10002/p2p/12D3KooWEiGVAFC7curXWXiGZyMWnZK9h8BKr88U8D5PKV3dXciv",

        "/dns4/webrtc2.webnode.openmina.com/tcp/8306/p2p/12D3KooWFpqySZDHx7k5FMjdwmrU3TLhDbdADECCautBcEGtG4fr",
        "/dns4/webrtc2.webnode.openmina.com/tcp/8307/p2p/12D3KooWJBeXosFxdBwe2mbKRjgRG69ERaUTpS9qo9NRkoE8kBpj",

        "/ip4/65.21.123.88/tcp/8302/p2p/12D3KooWLKSM9oHWU7qwL7Ci75wunkjXpRmK6j5xq527zGw554AF",
        "/ip4/65.109.123.166/tcp/8302/p2p/12D3KooWGc9vwL9DUvoLdBFPSQGCT2QTULskzhmXcn8zg2j3jdFF",
        "/ip4/176.9.64.21/tcp/8302/p2p/12D3KooWG9owTshte2gR3joP4sgwAfdoV9bQeeB5y9R3QUprKLdJ",
        "/ip4/35.238.71.15/tcp/65454/p2p/12D3KooWHdUVpCZ9KcF5hNBrwf2uy7BaPDKrxyHJAaM5epJgQucX",
        "/ip4/35.224.199.118/tcp/25493/p2p/12D3KooWGbjV7ptpzLu4BuykKfEsF4ebLyR8gZAMUissMToKGVDQ",
        "/ip4/35.193.28.252/tcp/37470/p2p/12D3KooWFcCiQqrzBVLEkPdpkHDgWr6AkSMthT96agKYBBVuRhHg",
        "/ip4/142.132.154.120/tcp/58654/p2p/12D3KooWMPxTu24mCpi3TwmkU4fJk7a8TQ4agFZeTHQRi8KCc3nj",
        "/ip4/65.108.121.245/tcp/8302/p2p/12D3KooWGQ4g2eY44n5JLqymi8KC55GbnujAFeXNQrmNKSq4NYrv",
        "/ip4/65.109.123.173/tcp/8302/p2p/12D3KooWMd8K8FFd76cacUEE6sSzUPr7wj71TvMqGdFSgrpv923k",
        "/ip4/65.109.123.235/tcp/8302/p2p/12D3KooWBK3vz1inMubXCUeDF4Min6eG5418toceG8QvNPWRW1Gz",
        "/ip4/34.172.208.246/tcp/46203/p2p/12D3KooWNafCBobFGSdJyYonvSCB5KDzW3JZYnVBF6q22yhcXGjM",
        "/ip4/34.29.40.184/tcp/7528/p2p/12D3KooWJoVjUsnDosW3Ae78V4CSf5SSe9Wyetr5DxutmMMfwdp8",
        "/ip4/34.122.249.235/tcp/55894/p2p/12D3KooWMpGyhYHbzVeqYnxGHQQYmQNtYcoMLLZZmYRPvAJKxXXm",
        "/ip4/35.232.20.138/tcp/10000/p2p/12D3KooWAdgYL6hv18M3iDBdaK1dRygPivSfAfBNDzie6YqydVbs",
        "/ip4/88.198.230.168/tcp/8302/p2p/12D3KooWGA7AS91AWNtGEBCBk64kgirtTiyaXDTyDtKPTjpefNL9",
        "/ip4/35.224.199.118/tcp/10360/p2p/12D3KooWDnC4XrJzas3heuz4LUehZjf2WJyfob2XEodrYL3soaf4",
        "/ip4/34.123.4.144/tcp/10002/p2p/12D3KooWEiGVAFC7curXWXiGZyMWnZK9h8BKr88U8D5PKV3dXciv",
        "/ip4/34.170.114.52/tcp/10001/p2p/12D3KooWLjs54xHzVmMmGYb7W5RVibqbwD1co7M2ZMfPgPm7iAag",
        "/ip4/34.172.208.246/tcp/54351/p2p/12D3KooWEhCm8FVcqZSkXKNhuBPmsEfJGeqSmUxNQhpemZkENfik",
        "/ip4/34.29.161.11/tcp/10946/p2p/12D3KooWCntSrMqSiovXcVfMZ56aYbzpZoh4mi7gJJNiZBmzXrpa",
        "/ip4/35.238.71.15/tcp/23676/p2p/12D3KooWENsfMszNYBRfHZJUEAvXKThmZU3nijWVbLivq33AE2Vk",
    ]
        .into_iter()
        .map(|s| s.parse().unwrap())
        .collect()
}

impl Snarker {
    pub fn run(self) -> Result<(), crate::CommandError> {
        tracing::initialize(self.verbosity);

        if let Err(ref e) = rayon::ThreadPoolBuilder::new()
            .num_threads(num_cpus::get().max(2) - 1)
            .thread_name(|i| format!("openmina_rayon_{i}"))
            .build_global()
        {
            shared::log::error!(shared::log::system_time();
                    kind = "FatalError",
                    summary = "failed to initialize threadpool",
                    error = format!("{:?}", e));
            panic!("FatalError: {:?}", e);
        }

        let rt = tokio::runtime::Builder::new_current_thread()
            .build()
            .unwrap();
        let _rt_guard = rt.enter();

        let mut rng = ThreadRng::default();
        let secret_key = std::env::var("OPENMINA_P2P_SEC_KEY")
            .ok()
            .filter(|s| !s.trim().is_empty())
            .map(|s| {
                s.trim()
                    .parse()
                    .expect("failed to parse `$OPENMINA_P2P_SEC_KEY` env")
            })
            .unwrap_or_else(|| SecretKey::from_bytes(rng.gen()));
        let pub_key = secret_key.public_key();

        let config = Config {
            ledger: LedgerConfig {},
            snark: SnarkConfig {
                // TODO(binier): use cache
                block_verifier_index: snarker::snark::get_verifier_index(VerifierKind::Blockchain)
                    .into(),
                block_verifier_srs: snarker::snark::get_srs().into(),
            },
            snarker: SnarkerConfig {
                public_key: self.public_key,
                fee: CurrencyFeeStableV1(UnsignedExtendedUInt64Int64ForVersionTagsStableV1(
                    self.fee.into(),
                )),
                job_commitments: SnarkPoolConfig {
                    commitment_timeout: Duration::from_secs(6 * 60),
                    auto_commit: self.auto_commit,
                },
                path: self.mina_exe_path,
            },
            p2p: P2pConfig {
                identity_pub_key: pub_key,
                initial_peers: self.peers,
                max_peers: 100,
                enabled_channels: [
                    ChannelId::BestTipPropagation,
                    ChannelId::SnarkPropagation,
                    ChannelId::SnarkJobCommitmentPropagation,
                    ChannelId::Rpc,
                ]
                .into(),
            },
            transition_frontier: TransitionFrontierConfig {
                protocol_constants: serde_json::from_value(serde_json::json!({
                    "k": "290",
                    "slots_per_epoch": "7140",
                    "slots_per_sub_window": "7",
                    "delta": "0",
                    // TODO(binier): fix wrong timestamp
                    "genesis_state_timestamp": "0",
                }))
                .unwrap(),
            },
        };
        let (event_sender, event_receiver) = mpsc::unbounded_channel();

        let (p2p_event_sender, mut rx) = mpsc::unbounded_channel::<P2pEvent>();

        let webrtc_rs_with_libp2p::P2pServiceCtx {
            libp2p,
            webrtc: P2pServiceCtx { cmd_sender, peers },
        } = <SnarkerService as P2pServiceWebrtcRsWithLibp2p>::init(
            secret_key,
            self.chain_id,
            p2p_event_sender.clone(),
            P2pTaskSpawner {},
        );

        let ev_sender = event_sender.clone();
        tokio::spawn(async move {
            while let Some(v) = rx.recv().await {
                if let Err(_) = ev_sender.send(v.into()) {
                    break;
                }
            }
        });

        let mut rpc_service = rpc::RpcService::new();

        let http_port = self.port;
        let rpc_sender = RpcSender {
            tx: rpc_service.req_sender().clone(),
        };

        // spawn http-server
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();
        std::thread::Builder::new()
            .name("openmina_http_server".to_owned())
            .spawn(move || {
                let local_set = tokio::task::LocalSet::new();
                local_set.block_on(&runtime, http_server::run(http_port, rpc_sender))
            })
            .unwrap();

        // spawn state machine thread.
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .thread_stack_size(64 * 1024 * 1024)
            .build()
            .unwrap();
        let (redux_exited_tx, redux_exited) = tokio::sync::oneshot::channel();
        std::thread::Builder::new()
            .name("openmina_redux".to_owned())
            .spawn(move || {
                let local_set = tokio::task::LocalSet::new();
                local_set.block_on(&runtime, async move {
                    let service = SnarkerService {
                        rng: ThreadRng::default(),
                        event_sender,
                        p2p_event_sender,
                        event_receiver: event_receiver.into(),
                        cmd_sender,
                        ledger: Default::default(),
                        peers,
                        libp2p,
                        rpc: rpc_service,
                        snark_worker_sender: None,
                        stats: Stats::new(),
                    };
                    let state = State::new(config);
                    let mut snarker = ::snarker::Snarker::new(state, service);

                    snarker
                        .store_mut()
                        .dispatch(EventSourceProcessEventsAction {});
                    snarker.
                        store_mut()
                        .dispatch(ExternalSnarkWorkerStartAction {});
                    loop {
                        snarker
                            .store_mut()
                            .dispatch(EventSourceWaitForEventsAction {});

                        let service = &mut snarker.store_mut().service;
                        let wait_for_events = service.event_receiver.wait_for_events();
                        let rpc_req_fut = async {
                            // TODO(binier): optimize maybe to not check it all the time.
                            match service.rpc.req_receiver().recv().await {
                                Some(v) => v,
                                None => std::future::pending().await,
                            }
                        };
                        let timeout = tokio::time::sleep(Duration::from_millis(100));

                        select! {
                            _ = wait_for_events => {
                                while snarker.store_mut().service.event_receiver.has_next() {
                                    snarker.store_mut().dispatch(EventSourceProcessEventsAction {});
                                }
                            }
                            req = rpc_req_fut => {
                                snarker.store_mut().service.process_rpc_request(req);
                                // TODO(binier): remove loop once ledger communication is async.
                                while let Ok(req) = snarker.store_mut().service.rpc.req_receiver().try_recv() {
                                    snarker.store_mut().service.process_rpc_request(req);
                                }
                            }
                            _ = timeout => {
                                snarker.store_mut().dispatch(EventSourceWaitTimeoutAction {});
                            }
                        }
                    }
                });
                let _ = redux_exited_tx.send(());
            })
            .unwrap();

        rt.block_on(redux_exited)
            .expect("state machine task crashed!");
        Ok(())
    }
}

struct SnarkerService {
    rng: ThreadRng,
    event_sender: mpsc::UnboundedSender<Event>,
    // TODO(binier): change so that we only have `event_sender`.
    p2p_event_sender: mpsc::UnboundedSender<P2pEvent>,
    event_receiver: EventReceiver,
    cmd_sender: mpsc::UnboundedSender<Cmd>,
    ledger: LedgerCtx,
    peers: BTreeMap<PeerId, PeerState>,
    libp2p: Libp2pService,
    rpc: rpc::RpcService,
    stats: Stats,
    snark_worker_sender: Option<ExternalSnarkWorkerSender>,
}

impl snarker::ledger::LedgerService for SnarkerService {
    fn ctx(&self) -> &LedgerCtx {
        &self.ledger
    }

    fn ctx_mut(&mut self) -> &mut LedgerCtx {
        &mut self.ledger
    }
}

impl redux::TimeService for SnarkerService {}

impl redux::Service for SnarkerService {}

impl snarker::Service for SnarkerService {
    fn stats(&mut self) -> Option<&mut Stats> {
        Some(&mut self.stats)
    }
}

impl EventSourceService for SnarkerService {
    fn next_event(&mut self) -> Option<Event> {
        self.event_receiver.try_next()
    }
}

impl P2pServiceWebrtcRs for SnarkerService {
    fn random_pick(
        &mut self,
        list: &[P2pConnectionOutgoingInitOpts],
    ) -> P2pConnectionOutgoingInitOpts {
        list.choose(&mut self.rng).unwrap().clone()
    }

    fn event_sender(&mut self) -> &mut mpsc::UnboundedSender<P2pEvent> {
        &mut self.p2p_event_sender
    }

    fn cmd_sender(&mut self) -> &mut mpsc::UnboundedSender<Cmd> {
        &mut self.cmd_sender
    }

    fn peers(&mut self) -> &mut BTreeMap<PeerId, PeerState> {
        &mut self.peers
    }
}

impl P2pServiceWebrtcRsWithLibp2p for SnarkerService {
    fn libp2p(&mut self) -> &mut Libp2pService {
        &mut self.libp2p
    }
}

impl SnarkBlockVerifyService for SnarkerService {
    fn verify_init(
        &mut self,
        req_id: SnarkBlockVerifyId,
        verifier_index: Arc<VerifierIndex>,
        verifier_srs: Arc<VerifierSRS>,
        block: VerifiableBlockWithHash,
    ) {
        let tx = self.event_sender.clone();
        rayon::spawn_fifo(move || {
            let header = block.header_ref();
            let result = {
                if !ledger::proofs::accumulator_check::accumulator_check(
                    &verifier_srs,
                    &header.protocol_state_proof,
                ) {
                    Err(SnarkBlockVerifyError::AccumulatorCheckFailed)
                } else if !ledger::proofs::verification::verify_block(
                    header,
                    &verifier_index,
                    &verifier_srs,
                ) {
                    Err(SnarkBlockVerifyError::VerificationFailed)
                } else {
                    Ok(())
                }
            };

            let _ = tx.send(SnarkEvent::BlockVerify(req_id, result).into());
        });
    }
}

pub struct EventReceiver {
    rx: mpsc::UnboundedReceiver<Event>,
    queue: Vec<Event>,
}

impl EventReceiver {
    /// If `Err(())`, `mpsc::Sender` for this channel was dropped.
    pub async fn wait_for_events(&mut self) -> Result<(), ()> {
        let next = self.rx.recv().await.ok_or(())?;
        self.queue.push(next);
        Ok(())
    }

    pub fn has_next(&mut self) -> bool {
        if self.queue.is_empty() {
            if let Some(event) = self.try_next() {
                self.queue.push(event);
                true
            } else {
                false
            }
        } else {
            true
        }
    }

    pub fn try_next(&mut self) -> Option<Event> {
        if !self.queue.is_empty() {
            Some(self.queue.remove(0))
        } else {
            self.rx.try_recv().ok()
        }
    }
}

impl From<mpsc::UnboundedReceiver<Event>> for EventReceiver {
    fn from(rx: mpsc::UnboundedReceiver<Event>) -> Self {
        Self {
            rx,
            queue: Vec::with_capacity(1),
        }
    }
}

pub struct SnarkerRpcRequest {
    pub req: RpcRequest,
    pub responder: Box<dyn Send + std::any::Any>,
}

#[derive(Clone)]
pub struct RpcSender {
    tx: mpsc::Sender<SnarkerRpcRequest>,
}

impl RpcSender {
    pub fn new(tx: mpsc::Sender<SnarkerRpcRequest>) -> Self {
        Self { tx }
    }

    pub async fn oneshot_request<T>(&self, req: RpcRequest) -> Option<T>
    where
        T: 'static + Send + Serialize,
    {
        let (tx, rx) = oneshot::channel::<T>();
        let responder = Box::new(tx);
        let sender = self.tx.clone();
        let _ = sender.send(SnarkerRpcRequest { req, responder }).await;

        rx.await.ok()
    }

    pub async fn multishot_request<T>(
        &self,
        expected_messages: usize,
        req: RpcRequest,
    ) -> mpsc::Receiver<T>
    where
        T: 'static + Send + Serialize,
    {
        let (tx, rx) = mpsc::channel::<T>(expected_messages);
        let responder = Box::new(tx);
        let sender = self.tx.clone();
        let _ = sender.send(SnarkerRpcRequest { req, responder }).await;

        rx
    }

    pub async fn peer_connect(
        &self,
        opts: P2pConnectionOutgoingInitOpts,
    ) -> Result<String, String> {
        let peer_id = opts.peer_id().to_string();
        let req = RpcRequest::P2pConnectionOutgoing(opts);
        self.oneshot_request::<RpcP2pConnectionOutgoingResponse>(req)
            .await
            .ok_or_else(|| "state machine shut down".to_owned())??;

        Ok(peer_id)
    }
}

#[derive(Clone)]
struct P2pTaskSpawner {}

impl TaskSpawner for P2pTaskSpawner {
    fn spawn_main<F>(&self, name: &str, fut: F)
    where
        F: 'static + Send + std::future::Future,
    {
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();
        std::thread::Builder::new()
            .name(format!("openmina_p2p_{name}"))
            .spawn(move || {
                let local_set = tokio::task::LocalSet::new();
                local_set.block_on(&runtime, fut);
            })
            .unwrap();
    }
}

#[derive(Debug, thiserror::Error)]
enum SnarkerError {
    #[error(transparent)]
    BinprotError(#[from] binprot::Error),
    #[error(transparent)]
    IOError(#[from] io::Error),
    #[error(transparent)]
    NixError(#[from] nix::Error),
    #[error("external snark worker is not running")]
    NotRunning,
    #[error("external snark worker is busy")]
    Busy,
    /// Protocol logic is broken
    #[error("communication is broken: {_0}")]
    Broken(String),
}

impl From<SnarkerError> for ExternalSnarkWorkerError {
    fn from(source: SnarkerError) -> Self {
        match source {
            SnarkerError::BinprotError(err) => {
                ExternalSnarkWorkerError::BinprotError(err.to_string())
            }
            SnarkerError::IOError(err) => ExternalSnarkWorkerError::IOError(err.to_string()),
            SnarkerError::NixError(err) => ExternalSnarkWorkerError::Error(format!("nix error: {err}")),
            SnarkerError::NotRunning => ExternalSnarkWorkerError::NotRunning,
            SnarkerError::Busy => ExternalSnarkWorkerError::Busy,
            SnarkerError::Broken(err) => ExternalSnarkWorkerError::Broken(err),
        }
    }
}

impl From<SnarkerError> for ExternalSnarkWorkerEvent {
    fn from(source: SnarkerError) -> Self {
        ExternalSnarkWorkerEvent::Error(source.into())
    }
}

async fn write_snark_work_spec<T: BinProtWrite, W: AsyncWrite + Unpin>(
    spec: T,
    mut w: W,
) -> Result<(), SnarkerError> {
    let mut buf = Vec::new();
    spec.binprot_write(&mut buf)?;
    let len = (buf.len() as u64).to_le_bytes();
    w.write_all(&len).await?;
    w.write_all(&buf).await?;
    Ok(())
}

async fn read_snark_work_result<R: AsyncRead + Unpin>(
    mut r: R,
) -> Result<SnarkWorkResult, SnarkerError> {
    let mut len_buf = [0; size_of::<u64>()];
    r.read_exact(&mut len_buf).await?;
    let len = u64::from_le_bytes(len_buf);

    let mut buf = Vec::with_capacity(len as usize);
    let mut r = r.take(len);
    r.read_to_end(&mut buf).await?;

    let mut read = buf.as_slice();
    let result = SnarkWorkerWorkerRpcsVersionedSubmitWorkV2TQuery::binprot_read(&mut read)?;
    Ok(Arc::new(result.proofs))
}

struct ExternalSnarkWorkerSender {
    data_chan: mpsc::Sender<SnarkWorkSpec>,
    cancel_chan: mpsc::Sender<()>,
    kill_chan: oneshot::Sender<()>,
}

impl ExternalSnarkWorkerSender {
    fn start<P: AsRef<OsStr>>(
        path: P,
        public_key: NonZeroCurvePoint,
        fee: CurrencyFeeStableV1,
        event_sender: mpsc::UnboundedSender<Event>,
    ) -> Result<Self, SnarkerError> {
        let (data_chan, mut data_rx) = mpsc::channel(1);
        let (cancel_chan, mut cancel_rx) = mpsc::channel(1);
        let (kill_chan, kill_rx) = oneshot::channel();

        let mut cmd = Command::new(path);

        std::thread::Builder::new()
            .name("external-snark-worker".into())
            .spawn(move || {
                let runtime = tokio::runtime::Builder::new_current_thread()
                    .enable_all()
                    .build()
                    .unwrap();
                runtime.block_on(async move {
                    let mut child = match cmd
                        .stdin(Stdio::piped())
                        .stdout(Stdio::piped())
                        .stderr(Stdio::inherit())
                        .spawn()
                    {
                        Ok(v) => v,
                        Err(err) => {
                            let _ = event_sender.send(ExternalSnarkWorkerEvent::Error(SnarkerError::from(err).into()).into());
                            return;
                        }
                    };

                    let _ = event_sender.send(ExternalSnarkWorkerEvent::Started.into());

                    let mut child_stdin = child.stdin.take().unwrap();
                    let mut child_stdout = child.stdout.take().unwrap();

                    let event_sender_clone = event_sender.clone();
                    if let Some(pid) = child.id() {
                        let pid = nix::unistd::Pid::from_raw(pid as i32);
                        tokio::spawn(async move {
                            loop {
                                let Some(spec) = data_rx.recv().await else {
                                    return;
                                };
                                let spec = SnarkWorkerWorkerRpcsVersionedGetWorkV2TResponse(Some((
                                    SnarkWorkerWorkerRpcsVersionedGetWorkV2TResponseA0 {
                                        instances: spec,
                                        fee: fee.clone(),
                                    },
                                    public_key.clone(),
                                )));

                                if let Err(err) = write_snark_work_spec(spec, &mut child_stdin).await {
                                    let _ = event_sender_clone.send(ExternalSnarkWorkerEvent::Error(SnarkerError::from(err).into()).into());
                                    return;
                                }
                                tokio::select! {
                                    _ = cancel_rx.recv() => {
                                        if let Err(err) = nix::sys::signal::kill(pid, nix::sys::signal::Signal::SIGINT) {
                                            let _ = event_sender_clone.send(ExternalSnarkWorkerEvent::Error(SnarkerError::from(err).into()).into());
                                            return;
                                        }
                                        let _ = event_sender_clone.send(ExternalSnarkWorkerEvent::WorkError(ExternalSnarkWorkerWorkError::Cancelled).into());
                                    }
                                    response = read_snark_work_result(&mut child_stdout) => {
                                        match response {
                                            Ok(result) => {
                                                let _ = event_sender_clone.send(ExternalSnarkWorkerEvent::WorkResult(result).into());
                                            }
                                            Err(err) => {
                                                let _ = event_sender_clone.send(ExternalSnarkWorkerEvent::WorkError(ExternalSnarkWorkerWorkError::Error(err.to_string())).into());
                                            }
                                        }
                                    }
                                }
                            }
                        });
                    }

                    tokio::select! {
                        _ = kill_rx => {
                            if let Err(err) = child.kill().await {
                                let _ = event_sender.send(ExternalSnarkWorkerEvent::Error(SnarkerError::from(err).into()).into());
                            } else {
                                let _ = event_sender.send(ExternalSnarkWorkerEvent::Killed.into());
                            }
                            return;
                        }
                        _ = child.wait() => {
                        }
                    };
                });
            })?;

        Ok(ExternalSnarkWorkerSender {
            data_chan,
            cancel_chan,
            kill_chan,
        })
    }

    fn cancel(&mut self) -> Result<(), SnarkerError> {
        self.cancel_chan
            .try_send(())
            .map_err(|_| SnarkerError::Broken("already cancelled".into()))
    }

    fn submit(&mut self, spec: SnarkWorkSpec) -> Result<(), SnarkerError> {
        self.data_chan
            .try_send(spec)
            .map_err(|_| SnarkerError::Busy)
    }

    fn kill(self) -> Result<(), SnarkerError> {
        self.kill_chan
            .send(())
            .map_err(|_| SnarkerError::Broken("already sent kill".into()))
    }
}

impl ExternalSnarkWorkerService for SnarkerService {
    fn start<P: AsRef<OsStr>>(
        &mut self,
        path: P,
        public_key: NonZeroCurvePoint,
        fee: CurrencyFeeStableV1,
    ) -> Result<(), snarker::external_snark_worker::ExternalSnarkWorkerError> {
        let cmd_sender =
            ExternalSnarkWorkerSender::start(path, public_key, fee, self.event_sender.clone())?;
        self.snark_worker_sender = Some(cmd_sender);
        Ok(())
    }

    fn submit(
        &mut self,
        spec: SnarkWorkSpec,
    ) -> Result<(), snarker::external_snark_worker::ExternalSnarkWorkerError> {
        self.snark_worker_sender
            .as_mut()
            .ok_or(SnarkerError::NotRunning)
            .and_then(|sender| sender.submit(spec))?;
        Ok(())
    }

    fn cancel(&mut self) -> Result<(), ExternalSnarkWorkerError> {
        self.snark_worker_sender
            .as_mut()
            .ok_or(SnarkerError::NotRunning)
            .and_then(|sender| sender.cancel())?;
        Ok(())
    }

    fn kill(&mut self) -> Result<(), snarker::external_snark_worker::ExternalSnarkWorkerError> {
        self.snark_worker_sender
            .take()
            .ok_or(SnarkerError::NotRunning)
            .and_then(|sender| sender.kill())?;
        Ok(())
    }
}

#[cfg(test)]
mod external_snark_worker_tests {
    use std::{env, ffi::OsString, path::Path};

    use binprot::BinProtRead;
    use mina_p2p_messages::v2::{
        CurrencyFeeStableV1, NonZeroCurvePoint, SnarkWorkerWorkerRpcsVersionedGetWorkV2TResponse,
        SnarkWorkerWorkerRpcsVersionedGetWorkV2TResponseA0,
    };
    use snarker::{event_source::Event, external_snark_worker::{ExternalSnarkWorkerEvent, SnarkWorkSpec, ExternalSnarkWorkerWorkError}};
    use tokio::sync::mpsc;

    use crate::commands::snarker::ExternalSnarkWorkerSender;

    macro_rules! expect_event {
        ($source:expr, $event:pat) => {
            let result = $source.recv().await.expect("failed to receive an event");
            let Event::ExternalSnarkWorker(result) = result else {
                panic!("unexpected event kind");
            };
            let $event = result else {
                panic!("unexpected snark worker event");
            };
        };
    }

    fn mina_exe_path() -> OsString {
        env::var_os("MINA_EXE_PATH")
            .or_else(|| {
                env::var_os("CARGO_MANIFEST_DIR")
                    .map(|dir| Path::new(&dir).join("bin/mina").into_os_string())
            })
            .unwrap()
    }

    #[tokio::test]
    async fn test_kill() {
        let (event_tx, mut event_rx) = mpsc::unbounded_channel();
        let cmd_sender = ExternalSnarkWorkerSender::start(
            mina_exe_path(),
            NonZeroCurvePoint::default(),
            CurrencyFeeStableV1(
                mina_p2p_messages::v2::UnsignedExtendedUInt64Int64ForVersionTagsStableV1(
                    10_i64.into(),
                ),
            ),
            event_tx,
        )
        .unwrap();

        expect_event!(event_rx, ExternalSnarkWorkerEvent::Started);

        cmd_sender.kill().expect("cannot kill worker");
        expect_event!(event_rx, ExternalSnarkWorkerEvent::Killed);
    }

    fn read_input<R: std::io::Read>(mut r: R) -> (NonZeroCurvePoint, CurrencyFeeStableV1, SnarkWorkSpec) {
        let SnarkWorkerWorkerRpcsVersionedGetWorkV2TResponse(Some((
            SnarkWorkerWorkerRpcsVersionedGetWorkV2TResponseA0 { instances, fee },
            public_key,
        ))) = SnarkWorkerWorkerRpcsVersionedGetWorkV2TResponse::binprot_read(&mut r)
            .expect("cannot read work spec")
        else {
            unreachable!("incorrect work spec");
        };

        (public_key, fee, instances)
    }

    #[tokio::test]
    async fn test_work() {
        const DATA: &[u8] = include_bytes!("../../../tests/files/snark_spec/spec1.bin");
        let mut r = DATA;
        let (public_key, fee, instances) = read_input(&mut r);

        let (event_tx, mut event_rx) = mpsc::unbounded_channel();
        let mut cmd_sender =
            ExternalSnarkWorkerSender::start(mina_exe_path(), public_key, fee, event_tx).unwrap();

        expect_event!(event_rx, ExternalSnarkWorkerEvent::Started);

        cmd_sender.submit(instances).unwrap();
        expect_event!(event_rx, ExternalSnarkWorkerEvent::WorkResult(_));

        cmd_sender.kill().expect("cannot kill worker");
        expect_event!(event_rx, ExternalSnarkWorkerEvent::Killed);
    }

    #[tokio::test]
    async fn test_cancel() {
        const DATA: &[u8] = include_bytes!("../../../tests/files/snark_spec/spec1.bin");
        let mut r = DATA;
        let (public_key, fee, instances) = read_input(&mut r);

        let (event_tx, mut event_rx) = mpsc::unbounded_channel();
        let mut cmd_sender =
            ExternalSnarkWorkerSender::start(mina_exe_path(), public_key, fee, event_tx).unwrap();

        expect_event!(event_rx, ExternalSnarkWorkerEvent::Started);

        cmd_sender.submit(instances.clone()).unwrap();
        expect_event!(event_rx, ExternalSnarkWorkerEvent::WorkResult(_));

        cmd_sender.cancel().unwrap();
        expect_event!(event_rx, ExternalSnarkWorkerEvent::WorkCancelled);

        cmd_sender.submit(instances).unwrap();
        expect_event!(event_rx, ExternalSnarkWorkerEvent::WorkResult(_));

        cmd_sender.kill().expect("cannot kill worker");
        expect_event!(event_rx, ExternalSnarkWorkerEvent::Killed);
    }
}
