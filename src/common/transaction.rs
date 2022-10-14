use serde::{Deserialize, Serialize};
use serde::de::Deserializer;
use named_type_derive::*;
use named_type::NamedType;
use devii::devii::FetchFields;
use getset::{CopyGetters, Getters, MutGetters};

use crate::common::block::Block;


#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug, Clone, NamedType, Default, Getters, CopyGetters, MutGetters)]
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
    
    #[getset(get = "pub", get_mut = "pub")]
    #[serde(alias = "transaction_amount_collection")]
    #[serde(rename(serialize = "transaction_amount_collection"))]
    #[serde(default)]
    transaction_amounts: Vec<TransactionAmount>
}

impl Transaction {
    pub fn new(hash: String, is_coinbase: bool, block: &Block) -> Self {
        Transaction {
            hash, 
            is_coinbase,
            date : block.date(),
            block_hash: block.hash().clone(),
            block_height: block.height(),
            transaction_amounts: vec![]
        }
    }
}

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug, Clone, NamedType, Default, Getters, CopyGetters)]
pub struct TransactionAmount {
    #[serde(deserialize_with = "deserialize_u64_or_string")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[getset(get_copy = "pub")]
    id: Option<u64>, // Primary Key

    #[getset(get_copy = "pub")]
    amount: f64,
    
    #[getset(get = "pub")]
    address: String, 
    
    #[getset(get = "pub")]
    transaction_hash: String,
    
    #[getset(get_copy = "pub")]
    index: u64
}

impl TransactionAmount {
    pub fn new(amount: f64, address: String, transaction_hash: String, index: u64) -> Self{
        TransactionAmount {
            id: None,
            amount,
            address,
            transaction_hash,
            index
        }
    }
}

// Credit : https://noyez.gitlab.io/post/2018-08-28-serilize-this-or-that-into-u64/
#[derive(Deserialize)]
#[serde(untagged)]
enum StringOrU64 { U64(u64), Str(String) }
pub fn deserialize_u64_or_string<'de, D>(deserializer: D) -> Result<Option<u64>, D::Error>
    where D: Deserializer<'de>
{
    match StringOrU64::deserialize(deserializer)? {
        StringOrU64::U64(v) => { Ok(Some(v)) }
        StringOrU64::Str(v) => {
            let res = v.parse::<u64>();
            if let Ok(r) = res {
                Ok(Some(r))
            } else {
                Err(serde::de::Error::custom("Can't parse id!"))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::common::transaction::{Transaction, TransactionAmount};
    use crate::common::block::Block;
    use serde_json;

    #[test]
    fn transaction_get_date_test() {
        let block = Block::new("hello_world".to_string(), 123456789, 420);
        let transaction = Transaction::new("hashy".to_string(), true, &block);

        assert_eq!(transaction.date(), 123456789);
    }

    #[test]
    fn transaction_get_hash_test() {
        let block = Block::new("hello_world".to_string(), 123456789, 420);
        let transaction = Transaction::new("hashy".to_string(), true, &block);

        assert_eq!(transaction.hash(), &"hashy".to_string());
    }

    #[test]
    fn transaction_get_is_coinbase_test() {
        let block = Block::new("hello_world".to_string(), 123456789, 420);
        let transaction = Transaction::new("hashy".to_string(), true, &block);

        assert_eq!(transaction.is_coinbase(), true);
    }

    #[test]
    fn transaction_get_block_hash_test() {
        let block = Block::new("hello_world".to_string(), 123456789, 420);
        let transaction = Transaction::new("hashy".to_string(), true, &block);

        assert_eq!(transaction.block_hash(), &"hello_world".to_string());
    }

    #[test]
    fn transaction_get_block_height_test() {
        let block = Block::new("hello_world".to_string(), 123456789, 420);
        let transaction = Transaction::new("hashy".to_string(), true, &block);

        assert_eq!(transaction.block_height(), 420);
    }

    #[test]
    fn transaction_amount_get_id_test() {
        let transaction_amount = TransactionAmount::new(90.8, "address".to_string(), "transaction_hash".to_string(), 5);

        assert_eq!(transaction_amount.id(), None);
    }

    #[test]
    fn transaction_amount_get_amount_test() {
        let transaction_amount = TransactionAmount::new(90.8, "address".to_string(), "transaction_hash".to_string(), 5);

        assert_eq!(transaction_amount.amount(), 90.8);
    }

    #[test]
    fn transaction_amount_get_address_test() {
        let transaction_amount = TransactionAmount::new(90.8, "address".to_string(), "transaction_hash".to_string(), 5);

        assert_eq!(transaction_amount.address(), &"address".to_string());
    }

    #[test]
    fn transaction_amount_get_transaction_hash_test() {
        let transaction_amount = TransactionAmount::new(90.8, "address".to_string(), "transaction_hash".to_string(), 5);

        assert_eq!(transaction_amount.transaction_hash(), &"transaction_hash".to_string());
    }

    #[test]
    fn transaction_amount_get_index_test() {
        let transaction_amount = TransactionAmount::new(90.8, "address".to_string(), "transaction_hash".to_string(), 5);

        assert_eq!(transaction_amount.index(), 5);
    }

    // With some software the primary key is always returned as a string so this is a check to make sure it deserializes back into an u64
    #[test]
    fn transaction_amount_deserialize_test_id_string() {
        let data = r#"
        {
            "id": "5", 
            "amount": 43.98,
            "transaction_hash": "hashy_transaction",
            "address" : "hashy_address",
            "index" : 42
        }"#;

        // Parse the string of data into serde_json::Value.
        let transaction_amount: Result<TransactionAmount, serde_json::Error> = serde_json::from_str(data);
        if let Ok(ta) = transaction_amount {
            assert_eq!(Some(5), ta.id());
            assert_eq!(43.98, ta.amount());
            assert_eq!(&"hashy_transaction".to_string(), ta.transaction_hash());
            assert_eq!(&"hashy_address".to_string(), ta.address());
            assert_eq!(42, ta.index());
        } else {
            println!("{:?}", transaction_amount);
            assert!(false);
        }
    }
   
    #[test]
    fn transaction_amount_deserialize_test_id_u64() {
        let data = r#"
        {
            "id": 5, 
            "amount": 43.98,
            "transaction_hash": "hashy_transaction",
            "address" : "hashy_address",
            "index" : 42
        }"#;

        // Parse the string of data into serde_json::Value.
        let transaction_amount: Result<TransactionAmount, serde_json::Error> = serde_json::from_str(data);
        if let Ok(ta) = transaction_amount {
            assert_eq!(Some(5), ta.id());
            assert_eq!(43.98, ta.amount());
            assert_eq!(&"hashy_transaction".to_string(), ta.transaction_hash());
            assert_eq!(&"hashy_address".to_string(), ta.address());
            assert_eq!(42, ta.index());
        } else {
            println!("{:?}", transaction_amount);
            assert!(false);
        }
    }

    #[test]
    fn transaction_deserialize_test() {
        let data = r#"
        {
            "hash": "hashy_hash", 
            "date": 123456789,
            "is_coinbase": false,
            "block_hash" : "hashy_block",
            "block_height" : 42069,
            "transaction_amount_collection" : [
                {
                    "id": 5, 
                    "amount": 43.98,
                    "transaction_hash": "hashy_transaction",
                    "address" : "hashy_address",
                    "index" : 42
                }
            ]
        }"#;

        // Parse the string of data into serde_json::Value.
        let transaction: Result<Transaction, serde_json::Error> = serde_json::from_str(data);
        if let Ok(t) = transaction {
            assert_eq!(&"hashy_hash".to_string(), t.hash());
            assert_eq!(123456789, t.date());
            assert_eq!(false, t.is_coinbase());
            assert_eq!(&"hashy_block".to_string(), t.block_hash());
            assert_eq!(42069, t.block_height());
            assert_eq!(1, t.transaction_amounts().len());
        } else {
            println!("{:?}", transaction);
            assert!(false);
        }
    }

    #[test]
    fn transaction_deserialize_test_no_transaction_amounts() {
        let data = r#"
        {
            "hash": "hashy_hash", 
            "date": 123456789,
            "is_coinbase": false,
            "block_hash" : "hashy_block",
            "block_height" : 42069
        }"#;

        // Parse the string of data into serde_json::Value.
        let transaction: Result<Transaction, serde_json::Error> = serde_json::from_str(data);
        if let Ok(t) = transaction {
            assert_eq!(&"hashy_hash".to_string(), t.hash());
            assert_eq!(123456789, t.date());
            assert_eq!(false, t.is_coinbase());
            assert_eq!(&"hashy_block".to_string(), t.block_hash());
            assert_eq!(42069, t.block_height());
            assert_eq!(0, t.transaction_amounts().len());
        } else {
            println!("{:?}", transaction);
            assert!(false);
        }
    }

    #[test]
    fn insert_amount_into_transaction_test() {
        let mut block = Block::new("hello_world".to_string(), 123456789, 420);
        let mut transaction = Transaction::new("hashy_transaction".to_string(), true, &block);
        let transaction_amount = TransactionAmount::new(99.9, "address".to_string(), "hashy_transaction".to_string(), 5);

        let amounts = transaction.transaction_amounts_mut();
        amounts.push(transaction_amount);

        assert_eq!(1, transaction.transaction_amounts().len());
    }
}