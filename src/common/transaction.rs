use serde::{Deserialize, Serialize};
use serde::de::Deserializer;
use named_type_derive::*;
use named_type::NamedType;
use std::cmp::Ordering;
use chrono::{Utc, SecondsFormat};
use serde_json::Value;
use devii::devii::DeviiTrait;
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
            last_updated: Utc::now().to_rfc3339_opts(SecondsFormat::Secs, true),
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

impl DeviiTrait for Transaction {
    fn fetch_fields() -> String {
        format!("{{ hash, is_coinbase, date, block_hash, block_height, last_updated, transaction_amounts {{ amount, address_hash, transaction_hash, date, index, last_updated }} }}")
    }
    fn insert_query(&self, param: String) -> String{
        format!("create_transactions (input: ${} ){{ hash }}", param)
    }
    fn input_type(&self) -> String {
        "transactionsInput".to_string()
    }
    fn graphql_inputs(&self) -> serde_json::Value {
        let value = serde_json::to_value(&self).unwrap();
        match value {
            Value::Object(mut map) => {
                map.remove_entry("transaction_amounts");
                map.remove_entry("transaction_amount_collection");
                return Value::Object(map)
            }, 
            _ => panic!("Transaction not an Object!"),
        }
    }
    fn delete_input(&self) -> String {
        format!("hash: \"{}\"", self.hash())
    }
}

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug, Clone, NamedType, Default, Getters, CopyGetters)]
pub struct TransactionAmount { 
    #[getset(get_copy = "pub")]
    amount: f64,
    
    #[getset(get = "pub")]
    address_hash: String, 
    
    #[getset(get = "pub")]
    transaction_hash: String,
    
    #[serde(deserialize_with = "deserialize_u32_or_string")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[getset(get_copy = "pub")]
    index: Option<u32>,

    #[getset(get_copy = "pub")]
    date: u64,

    #[getset(get_copy = "pub")]
    vin_index: i64, 

    #[getset(get = "pub")]
    vin_hash: Option<String>
}

impl TransactionAmount {
    pub fn new(amount: f64, address_hash: String, transaction_hash: String, date: u64, index: Option<u32>) -> Self{
        TransactionAmount {
            amount,
            address_hash,
            transaction_hash,
            date,
            index,
            vin_index: -1,
            vin_hash: None
        }
    }
}

impl DeviiTrait for TransactionAmount {
    fn fetch_fields() -> String {
        format!("{{ amount, address_hash, transaction_hash, date, index, vin_index, vin_hash }}")
    }
    fn insert_query(&self, param: String) -> String{
        format!("create_transaction_amounts (input: ${} ){{ transaction_hash, index, vin_index }}", param)
    }
    fn input_type(&self) -> String {
        "transaction_amountsInput".to_string()
    }
    fn graphql_inputs(&self) -> serde_json::Value {
        serde_json::to_value(&self).unwrap()
    }
    fn delete_input(&self) -> String {
        format!("transaction_hash: \"{}\", index: \"{}\", vin_index: \"{}\"", self.transaction_hash(), self.index().unwrap(), self.vin_index())
    }
}

// Credit : https://noyez.gitlab.io/post/2018-08-28-serilize-this-or-that-into-u64/
#[derive(Deserialize)]
#[serde(untagged)]
enum StringOrU32 { U32(u32), Str(String) }
pub fn deserialize_u32_or_string<'de, D>(deserializer: D) -> Result<Option<u32>, D::Error>
    where D: Deserializer<'de>
{
    match StringOrU32::deserialize(deserializer)? {
        StringOrU32::U32(v) => { Ok(Some(v)) }
        StringOrU32::Str(v) => {
            let res = v.parse::<u32>();
            if let Ok(r) = res {
                Ok(Some(r))
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
    use devii::devii::DeviiTrait;
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
        let transaction_amount = TransactionAmount::new(90.8, "address".to_string(), "transaction_hash".to_string(), 123456789, Some(5));
        
        assert_eq!(transaction_amount.amount(), 90.8);
        assert_eq!(transaction_amount.address_hash(), &"address".to_string());
        assert_eq!(transaction_amount.transaction_hash(), &"transaction_hash".to_string());
        assert_eq!(transaction_amount.index(), Some(5));
        assert_eq!(transaction_amount.date(), 123456789);
    }



    #[test]
    fn insert_amount_into_transaction_test() {
        let block = Block::new("hello_world".to_string(), 123456789, 420);
        let mut transaction = Transaction::new_from_block("hashy_transaction".to_string(), true, &block);
        let transaction_amount = TransactionAmount::new(99.9, "address".to_string(), "hashy_transaction".to_string(), 123456789, Some(5));

        let amounts = transaction.transaction_amounts_mut();
        amounts.push(transaction_amount);

        assert_eq!(1, transaction.transaction_amounts().len());
    }

    #[test]
    fn set_transaction_amounts_test() {
        let block = Block::new("hello_world".to_string(), 123456789, 420);
        let mut transaction = Transaction::new_from_block("hashy_transaction".to_string(), true, &block);
        let transaction_amount = TransactionAmount::new(99.9, "address".to_string(), "hashy_transaction".to_string(), 123456789, Some(5));

        transaction.set_transaction_amounts(vec![transaction_amount]);

        assert_eq!(1, transaction.transaction_amounts().len());
    }

    #[test]
    fn implements_devii_trait() {
        let block = Block::new("hello_world".to_string(), 123456789, 420);
        fn devii<T: DeviiTrait>(o: T) -> () {
            println!("{:?}", "o");
        }
        devii(block);
        assert!(true)
    }
}