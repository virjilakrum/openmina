use redux::{ActionMeta, Timestamp};
use serde::{Deserialize, Serialize};

pub use crate::job_commitment::JobCommitmentsState;
pub use crate::p2p::P2pState;
pub use crate::rpc::RpcState;
use crate::ActionWithMeta;
pub use crate::Config;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct State {
    pub p2p: P2pState,
    pub job_commitments: JobCommitmentsState,
    pub rpc: RpcState,

    // TODO(binier): include action kind in `last_action`.
    pub last_action: ActionMeta,
    pub applied_actions_count: u64,
}

impl State {
    pub fn new(config: Config) -> Self {
        Self {
            p2p: P2pState::new(config.p2p),
            job_commitments: JobCommitmentsState::new(),
            rpc: RpcState::new(),

            last_action: ActionMeta::ZERO,
            applied_actions_count: 0,
        }
    }

    #[inline(always)]
    pub fn time(&self) -> Timestamp {
        self.last_action.time()
    }

    pub fn action_applied(&mut self, action: &ActionWithMeta) {
        self.last_action = action.meta().clone();
        self.applied_actions_count += 1;
    }
}
