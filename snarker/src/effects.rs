use crate::consensus::consensus_effects;
use crate::event_source::event_source_effects;
use crate::external_snark_worker::external_snark_worker_effects;
use crate::logger::logger_effects;
use crate::p2p::channels::rpc::{
    P2pChannelsRpcRequestSendAction, P2pChannelsRpcTimeoutAction, P2pRpcKind, P2pRpcRequest,
};
use crate::p2p::connection::outgoing::{
    P2pConnectionOutgoingRandomInitAction, P2pConnectionOutgoingReconnectAction,
};
use crate::p2p::p2p_effects;
use crate::rpc::rpc_effects;
use crate::snark::snark_effects;
use crate::snark_pool::{
    job_commitment_effects, SnarkPoolCheckTimeoutsAction, SnarkPoolP2pSendAllAction,
};
use crate::transition_frontier::sync::TransitionFrontierSyncBlocksNextApplyInitAction;
use crate::transition_frontier::transition_frontier_effects;
use crate::watched_accounts::watched_accounts_effects;
use crate::{Action, ActionWithMeta, Service, Store};

pub fn effects<S: Service>(store: &mut Store<S>, action: ActionWithMeta) {
    let (action, meta) = action.split();

    if let Some(stats) = store.service.stats() {
        stats.new_action(action.kind(), meta.clone());
    }

    logger_effects(store, meta.clone().with_action(&action));
    match action {
        Action::CheckTimeouts(_) => {
            store.dispatch(P2pConnectionOutgoingRandomInitAction {});

            let reconnect_actions: Vec<_> = store
                .state()
                .p2p
                .peers
                .iter()
                .filter_map(|(_, p)| p.dial_opts.clone())
                .map(|opts| P2pConnectionOutgoingReconnectAction { opts, rpc_id: None })
                .collect();
            for action in reconnect_actions {
                store.dispatch(action);
            }

            store.dispatch(SnarkPoolCheckTimeoutsAction {});
            store.dispatch(SnarkPoolP2pSendAllAction {});

            // TODO(binier): refactor
            let state = store.state();
            let consensus_best_tip_hash = state.consensus.best_tip.as_ref();
            let best_tip_hash = state.transition_frontier.best_tip().map(|v| &v.hash);
            let syncing_best_tip_hash = state.transition_frontier.sync.best_tip().map(|v| &v.hash);

            if consensus_best_tip_hash.is_some()
                && consensus_best_tip_hash != best_tip_hash
                && consensus_best_tip_hash != syncing_best_tip_hash
                && state.consensus.best_tip_chain_proof.is_none()
            {
                if !state
                    .p2p
                    .ready_peers_iter()
                    .filter_map(|(_, s)| s.channels.rpc.pending_local_rpc_kind())
                    .any(|kind| matches!(kind, P2pRpcKind::BestTipWithProof))
                {
                    // TODO(binier): choose randomly.
                    if let Some((peer_id, id)) = state
                        .p2p
                        .ready_peers_iter()
                        .filter(|(_, p)| p.channels.rpc.can_send_request())
                        .filter(|(_, p)| {
                            p.best_tip
                                .as_ref()
                                .map_or(true, |b| Some(b.hash()) == consensus_best_tip_hash)
                        })
                        .map(|(peer_id, p)| (*peer_id, p.channels.rpc.next_local_rpc_id()))
                        .last()
                    {
                        store.dispatch(P2pChannelsRpcRequestSendAction {
                            peer_id,
                            id,
                            request: P2pRpcRequest::BestTipWithProof,
                        });
                    }
                }
            }

            // Every minute.
            if (meta.time_as_nanos() / 10_000_000_000) % 10 == 6 {
                let state = store.state();
                let should_request = state
                    .transition_frontier
                    .sync
                    .best_tip()
                    .or_else(|| state.transition_frontier.best_tip())
                    .map_or(true, |b| {
                        let dur = meta.time().checked_sub(b.timestamp());
                        dur.map_or(false, |dur| dur.as_secs() >= 3 * 60)
                    });

                if should_request
                    && !state
                        .p2p
                        .ready_peers_iter()
                        .filter_map(|(_, s)| s.channels.rpc.pending_local_rpc_kind())
                        .any(|kind| matches!(kind, P2pRpcKind::BestTipWithProof))
                {
                    // TODO(binier): choose randomly.
                    let peers = state.p2p.ready_rpc_peers_iter().collect::<Vec<_>>();
                    for (peer_id, id) in peers {
                        store.dispatch(P2pChannelsRpcRequestSendAction {
                            peer_id,
                            id,
                            request: P2pRpcRequest::BestTipWithProof,
                        });
                    }
                }
            }

            let state = store.state();
            for (peer_id, id) in state.p2p.peer_rpc_timeouts(state.time()) {
                store.dispatch(P2pChannelsRpcTimeoutAction { peer_id, id });
            }

            // TODO(binier): remove once ledger communication is async.
            store.dispatch(TransitionFrontierSyncBlocksNextApplyInitAction {});
        }
        Action::EventSource(action) => {
            event_source_effects(store, meta.with_action(action));
        }
        Action::Snark(action) => {
            snark_effects(store, meta.with_action(action));
        }
        Action::Consensus(action) => {
            consensus_effects(store, meta.with_action(action));
        }
        Action::TransitionFrontier(action) => {
            transition_frontier_effects(store, meta.with_action(action));
        }
        Action::P2p(action) => {
            p2p_effects(store, meta.with_action(action));
        }
        Action::SnarkPool(action) => {
            job_commitment_effects(store, meta.with_action(action));
        }
        Action::Rpc(action) => {
            rpc_effects(store, meta.with_action(action));
        }
        Action::ExternalSnarkWorker(action) => {
            external_snark_worker_effects(store, meta.with_action(action));
        }
        Action::WatchedAccounts(action) => {
            watched_accounts_effects(store, meta.with_action(action));
        }
    }
}
