use serde::{Deserialize, Serialize};
use named_type_derive::*;
use named_type::NamedType;
// use devii::devii::FetchFields;

use crate::common::transaction::{Transaction};

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug, Clone, NamedType, Default)]
pub struct Address {
    hash: String, // Primary Key 5 bytes
    last_transaction: u32, // 4 bytes
    coin_total: f64, // 8 bytes 
    file_url: String, // 7 bytes 

    // Stored in Blob Storage
    first_transaction: u32,
    is_miner: bool,
    last_updated: u32,
    transactions: Option<Vec<Transaction>> 
}