use crate::state::PendingTransfer;
use near_sdk::borsh::{BorshDeserialize, BorshSerialize};
use near_sdk::store::{LazyOption, LookupMap};
use near_sdk::AccountId;
use near_sdk_macros::NearSchema;

// State for version 0.1.0
#[derive(BorshSerialize, BorshDeserialize, NearSchema)]
#[abi(borsh)]
pub struct StateV010 {
    pub version: String,
    pub manager: AccountId,
    pub offload_recipient: AccountId,
    pub auth_contract: AccountId,
    pub ft_wrapper_contract: AccountId,
    pub omni_locker_contract: LazyOption<AccountId>,
    pub chain_mpc_mapping: LookupMap<String, AccountId>,
    pub sponsor_amount: u128,
    pub sponsor_gas: u64,
    pub cross_contract_gas: u64,
    pub migration_gas: u64,
    pub chunk_size: usize,
    pub min_balance: u128,
    pub max_balance: u128,
    pub base_fee: u128,
    pub transfer_nonces: LookupMap<String, u64>,
    pub pending_transfers: LookupMap<String, PendingTransfer>,
}
