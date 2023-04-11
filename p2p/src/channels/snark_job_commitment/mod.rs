mod p2p_channels_snark_job_commitment_state;
pub use p2p_channels_snark_job_commitment_state::*;

mod p2p_channels_snark_job_commitment_actions;
pub use p2p_channels_snark_job_commitment_actions::*;

mod p2p_channels_snark_job_commitment_reducer;
pub use p2p_channels_snark_job_commitment_reducer::*;

mod p2p_channels_snark_job_commitment_effects;
pub use p2p_channels_snark_job_commitment_effects::*;

use binprot_derive::{BinProtRead, BinProtWrite};
use mina_p2p_messages::v2::{LedgerHash, MinaBaseSignatureStableV1, NonZeroCurvePoint};
use redux::Timestamp;
use serde::{Deserialize, Serialize};

#[derive(BinProtWrite, BinProtRead, Serialize, Deserialize, Debug, Clone)]
pub enum SnarkJobCommitmentPropagationChannelMsg {
    /// Request next commitments upto the `limit`.
    ///
    /// - Must not be sent until peer sends `WillSend` message for the
    ///   previous request and until peer has fulfilled it.
    GetNext { limit: u8 },
    /// Amount of commitments which will proceed this message.
    ///
    /// - Can only be sent, if peer has sent `GetNext` and we haven't
    ///   responded with `WillSend` yet.
    /// - Can't be bigger than limit set by `GetNext`.
    /// - Amount of promised commitments must be delivered.
    WillSend { count: u8 },
    /// Promise/Commitments from the snark worker to produce a proof.
    Commitment(SnarkJobCommitment),
}

#[derive(BinProtWrite, BinProtRead, Serialize, Deserialize, Debug, Clone)]
pub struct SnarkJobCommitment {
    /// Timestamp in milliseconds.
    timestamp: u32,
    pub job_id: SnarkJobId,
    pub snarker: NonZeroCurvePoint,
    pub signature: MinaBaseSignatureStableV1,
}

impl SnarkJobCommitment {
    pub fn new(timestamp: u32, job_id: SnarkJobId, snarker: NonZeroCurvePoint) -> Self {
        Self {
            timestamp,
            job_id,
            snarker,
            signature: todo!(),
        }
    }

    pub fn timestamp(&self) -> Timestamp {
        Timestamp::new(self.timestamp as u64 * 1_000_000)
    }
}

#[derive(
    BinProtWrite, BinProtRead, Serialize, Deserialize, Debug, Ord, PartialOrd, Eq, PartialEq, Clone,
)]
pub struct SnarkJobId {
    pub source: SnarkJobLedgerHashes,
    pub target: SnarkJobLedgerHashes,
}

#[derive(
    BinProtWrite, BinProtRead, Serialize, Deserialize, Debug, Ord, PartialOrd, Eq, PartialEq, Clone,
)]
pub struct SnarkJobLedgerHashes {
    pub first_pass_ledger: LedgerHash,
    pub second_pass_ledger: LedgerHash,
}
