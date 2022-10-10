use serde::{Deserialize, Serialize};
use named_type_derive::*;
use named_type::NamedType;
use devii::devii::FetchFields;

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug, Clone, NamedType, Default)]
pub struct Transaction {
    hash: String,
    date: u32,
    is_coinbase: bool, 
    block_hash: String,
    block_height: u64,
    amounts: Option<Vec<TransactionAmount>>
}

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug, Clone, NamedType, Default)]
pub struct TransactionAmount {
    id: Option<u64>,
    amount: u64,
    address: String, 
    transaction_hash: String,
    index: u64
}