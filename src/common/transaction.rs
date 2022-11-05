use serde::{Deserialize, Serialize};
use serde::de::Deserializer;
use named_type_derive::*;
use named_type::NamedType;
use std::cmp::Ordering;
use chrono::{Utc};

// use devii::devii::FetchFields;
use getset::{CopyGetters, Getters, MutGetters, Setters};

use crate::common::block::Block;


#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug, Clone, NamedType, Default, Getters, CopyGetters, MutGetters, Setters)]
pub struct Transaction {
    #[getset(get = "pub")]
    hash: String, // Primary Key
    
    #[getset(get_copy = "pub")]
    date: u64,

    #[getset(get_copy = "pub")]
    is_coinbase: bool, 
    
    #[getset(get = "pub")]
    block_hash: String,
    
    #[getset(get_copy = "pub")]
    block_height: u64,
    
    #[getset(get = "pub")]
    last_updated: String,

    #[getset(get = "pub", get_mut = "pub", set = "pub")]
    #[serde(alias = "transaction_amount_collection")]
    #[serde(rename(serialize = "transaction_amount_collection"))]
    #[serde(default)]
    transaction_amounts: Vec<TransactionAmount>
}

impl Transaction {
    pub fn new(hash: String, is_coinbase: bool, date: u64, block_hash: String, block_height: u64) -> Self {
        Transaction {
            hash,
            is_coinbase,
            date,
            block_hash,
            block_height,
            last_updated: Utc::now().to_string(),
            transaction_amounts: vec![]
        }
    }
    pub fn new_from_block(hash: String, is_coinbase: bool, block: &Block) -> Self {
        Transaction {
            hash, 
            is_coinbase,
            date : block.date(),
            block_hash: block.hash().clone(),
            block_height: block.height(),
            last_updated: Utc::now().to_string(),
            transaction_amounts: vec![]
        }
    }
}

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug, Clone, NamedType, Default, Getters, CopyGetters)]
pub struct TransactionAmount { 
    #[getset(get_copy = "pub")]
    amount: f64,
    
    #[getset(get = "pub")]
    address: String, 
    
    #[getset(get = "pub")]
    transaction_hash: String,
    
    #[serde(deserialize_with = "deserialize_u64_or_string")]
    #[getset(get_copy = "pub")]
    index: u64,

    #[getset(get_copy = "pub")]
    date: u64,

    #[getset(get = "pub")]
    last_updated: String
}

impl TransactionAmount {
    pub fn new(amount: f64, address: String, transaction_hash: String, date: u64, index: u64) -> Self{
        TransactionAmount {
            amount,
            address,
            transaction_hash,
            date,
            index,
            last_updated: Utc::now().to_string()
        }
    }
}

// Credit : https://noyez.gitlab.io/post/2018-08-28-serilize-this-or-that-into-u64/
#[derive(Deserialize)]
#[serde(untagged)]
enum StringOrU64 { U64(u64), Str(String) }
pub fn deserialize_u64_or_string<'de, D>(deserializer: D) -> Result<u64, D::Error>
    where D: Deserializer<'de>
{
    match StringOrU64::deserialize(deserializer)? {
        StringOrU64::U64(v) => { Ok(v) }
        StringOrU64::Str(v) => {
            let res = v.parse::<u64>();
            if let Ok(r) = res {
                Ok(r)
            } else {
                Err(serde::de::Error::custom("Can't parse id!"))
            }
        }
    }
}

impl Ord for TransactionAmount {
    fn cmp(&self, other: &Self) -> Ordering {
        self.date.cmp(&other.date)
    }
}

impl PartialOrd for TransactionAmount {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for TransactionAmount {
    fn eq(&self, other: &Self) -> bool {
        self.date == other.date
    }
}
impl Eq for TransactionAmount {}

#[cfg(test)]
mod tests {
    use crate::common::transaction::{Transaction, TransactionAmount};
    use crate::common::block::Block;

    #[test]
    fn transaction_get_date_test() {
        let block = Block::new("hello_world".to_string(), 123456789, 420);
        let transaction = Transaction::new_from_block("hashy".to_string(), true, &block);

        assert_eq!(transaction.date(), 123456789);
    }
    #[test]
    fn transaction_new_test() {
        let transaction = Transaction::new("hashy".to_string(), true, 123456789, "hello_world".to_string(), 420);

        assert_eq!(transaction.hash(), &"hashy".to_string());
        assert_eq!(transaction.is_coinbase(), true);
        assert_eq!(transaction.block_hash(), &"hello_world".to_string());
        assert_eq!(transaction.date(), 123456789);
        assert_eq!(transaction.block_height(), 420);
    }

    #[test]
    fn transaction_get_hash_test() {
        let block = Block::new("hello_world".to_string(), 123456789, 420);
        let transaction = Transaction::new_from_block("hashy".to_string(), true, &block);

        assert_eq!(transaction.hash(), &"hashy".to_string());
    }

    #[test]
    fn transaction_get_is_coinbase_test() {
        let block = Block::new("hello_world".to_string(), 123456789, 420);
        let transaction = Transaction::new_from_block("hashy".to_string(), true, &block);

        assert_eq!(transaction.is_coinbase(), true);
    }

    #[test]
    fn transaction_get_block_hash_test() {
        let block = Block::new("hello_world".to_string(), 123456789, 420);
        let transaction = Transaction::new_from_block("hashy".to_string(), true, &block);

        assert_eq!(transaction.block_hash(), &"hello_world".to_string());
    }

    #[test]
    fn transaction_get_block_height_test() {
        let block = Block::new("hello_world".to_string(), 123456789, 420);
        let transaction = Transaction::new_from_block("hashy".to_string(), true, &block);

        assert_eq!(transaction.block_height(), 420);
    }    
    
    #[test]
    fn transaction_amount_get_tests() {
        let transaction_amount = TransactionAmount::new(90.8, "address".to_string(), "transaction_hash".to_string(), 123456789, 5);
        
        assert_eq!(transaction_amount.amount(), 90.8);
        assert_eq!(transaction_amount.address(), &"address".to_string());
        assert_eq!(transaction_amount.transaction_hash(), &"transaction_hash".to_string());
        assert_eq!(transaction_amount.index(), 5);
        assert_eq!(transaction_amount.date(), 123456789);
    }



    #[test]
    fn insert_amount_into_transaction_test() {
        let block = Block::new("hello_world".to_string(), 123456789, 420);
        let mut transaction = Transaction::new_from_block("hashy_transaction".to_string(), true, &block);
        let transaction_amount = TransactionAmount::new(99.9, "address".to_string(), "hashy_transaction".to_string(), 123456789, 5);

        let amounts = transaction.transaction_amounts_mut();
        amounts.push(transaction_amount);

        assert_eq!(1, transaction.transaction_amounts().len());
    }

    #[test]
    fn set_transaction_amounts_test() {
        let block = Block::new("hello_world".to_string(), 123456789, 420);
        let mut transaction = Transaction::new_from_block("hashy_transaction".to_string(), true, &block);
        let transaction_amount = TransactionAmount::new(99.9, "address".to_string(), "hashy_transaction".to_string(), 123456789, 5);

        transaction.set_transaction_amounts(vec![transaction_amount]);

        assert_eq!(1, transaction.transaction_amounts().len());
    }
}