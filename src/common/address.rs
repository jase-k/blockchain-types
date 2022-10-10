use serde::{Deserialize, Serialize};
use named_type_derive::*;
use named_type::NamedType;
use devii::devii::FetchFields;

use crate::common::transaction::{Transaction};

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug, Clone, NamedType, Default)]
pub struct Address {
    hash: String,
    last_transaction: u32,
    first_transaction: u32,
    coin_total: u64,
    // tx_ids: Vec<Transactions>,
    is_miner: bool,
    last_updated: u32,
    transactions: Option<Vec<Transaction>>
}