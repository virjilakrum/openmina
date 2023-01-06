use super::{
    account_relevant_transactions_in_diff_iter, WatchedAccountBlockInfo, WatchedAccountBlockState,
    WatchedAccountState, WatchedAccountsAction, WatchedAccountsActionWithMetaRef,
    WatchedAccountsState,
};

impl WatchedAccountsState {
    pub fn reducer(&mut self, action: WatchedAccountsActionWithMetaRef<'_>) {
        let (action, _) = action.split();
        match action {
            WatchedAccountsAction::Add(action) => {
                self.insert(
                    action.pub_key.clone(),
                    WatchedAccountState {
                        blocks: Default::default(),
                    },
                );
            }
            WatchedAccountsAction::TransactionsIncludedInBlock(action) => {
                let block = &action.block;
                let header = &block.block.header;
                let diff = &block.block.body.staged_ledger_diff.diff;

                let transactions =
                    account_relevant_transactions_in_diff_iter(&action.pub_key, diff).collect();

                let Some(account) = self.get_mut(&action.pub_key) else { return };
                account
                    .blocks
                    .push_back(WatchedAccountBlockState::TransactionsInBlockBody {
                        block: WatchedAccountBlockInfo {
                            level: header
                                .protocol_state
                                .body
                                .consensus_state
                                .blockchain_length
                                .0
                                 .0 as u32,
                            hash: block.hash.clone(),
                            pred_hash: header.protocol_state.previous_state_hash.clone(),
                            staged_ledger_hash: header
                                .protocol_state
                                .body
                                .blockchain_state
                                .staged_ledger_hash
                                .non_snark
                                .ledger_hash
                                .clone(),
                        },
                        transactions,
                    });
            }
            WatchedAccountsAction::BlockLedgerQueryInit(_) => {}
            WatchedAccountsAction::BlockLedgerQueryPending(action) => {
                let Some(account) = self.get_mut(&action.pub_key) else { return };
                let Some(block_state) = account.block_find_by_hash_mut(&action.block_hash) else { return };
                *block_state = match block_state {
                    WatchedAccountBlockState::TransactionsInBlockBody {
                        block,
                        transactions,
                    } => WatchedAccountBlockState::LedgerAccountGetPending {
                        block: block.clone(),
                        transactions: std::mem::take(transactions),
                        p2p_rpc_id: action.p2p_rpc_id,
                    },
                    _ => return,
                };
            }
            WatchedAccountsAction::BlockLedgerQuerySuccess(action) => {
                let Some(account) = self.get_mut(&action.pub_key) else { return };
                let Some(block_state) = account.block_find_by_hash_mut(&action.block_hash) else { return };
                *block_state = match block_state {
                    WatchedAccountBlockState::LedgerAccountGetPending {
                        block,
                        transactions,
                        ..
                    } => WatchedAccountBlockState::LedgerAccountGetSuccess {
                        block: block.clone(),
                        transactions: std::mem::take(transactions),
                        ledger_account: action.ledger_account.clone(),
                    },
                    _ => return,
                };
            }
        }
    }
}
