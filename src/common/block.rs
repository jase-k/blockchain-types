use serde::{Deserialize, Serialize};
use named_type_derive::*;
use named_type::NamedType;
use devii::devii::FetchFields;

use crate::common::transaction::{Transaction};

#[derive(Serialize, Deserialize, Debug, Clone, NamedType, Default)]
pub struct Block {
    hash: String,
    date: u64,
    height: u64,
    transactions: Option<Transaction>
}