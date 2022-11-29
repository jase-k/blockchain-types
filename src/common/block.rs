use serde::{Deserialize, Serialize};
use named_type_derive::*;
use named_type::NamedType;
use serde_json::Value;
use devii::devii::DeviiTrait;
use getset::{CopyGetters, Getters, MutGetters, Setters};
use chrono::{Utc, SecondsFormat};

use crate::common::transaction::{Transaction};

#[derive(Serialize, Deserialize, Debug, Clone, NamedType, Default, Getters, CopyGetters, MutGetters, Setters)]
pub struct Block {
    #[getset(get = "pub")]
    hash: String, // Primary Key
    
    #[getset(get_copy = "pub")]
    #[serde(alias = "time")]
    date: u64,

    #[getset(get_copy = "pub")]
    height: u64,

    #[getset(get_copy = "pub")]
    is_final: bool,

    #[getset(get = "pub")]
    last_updated: String,
    
    #[getset(get = "pub", get_mut = "pub", set = "pub")]
    #[serde(alias = "transaction_collection")]
    #[serde(rename(serialize = "transaction_collection"))]
    #[serde(default)]
    transactions: Vec<Transaction>
}

impl Block {
    pub fn new(hash: String, date: u64, height: u64) -> Self {
        Block {
            hash,
            date, 
            height,
            is_final: false,
            last_updated: Utc::now().to_rfc3339_opts(SecondsFormat::Secs, true),
            transactions: vec![]
        }
    }
}
impl DeviiTrait for Block {
    fn fetch_fields() -> String {
        format!("{{ hash, date, height, is_final, last_updated, transaction_collection {{ hash, is_coinbase, date, block_hash, block_height, last_updated, transaction_amount_collection {{ amount, address_hash, transaction_hash, date, index, vin_index, vin_hash }} }} }}")
    }
    fn insert_query(&self, param: String) -> String{
        format!("create_blocks (input: ${} ){{ hash }}", param)
    }
    fn input_type(&self) -> String {
        "blocksInput".to_string()
    }
    fn graphql_inputs(&self) -> serde_json::Value {
        let value = serde_json::to_value(&self).unwrap();
        match value {
            Value::Object(mut map) => {
                map.remove_entry("transactions");
                map.remove_entry("transaction_collection");
                return Value::Object(map)
            }, 
            _ => panic!("Block not an Object!"),
        }
    }
    fn delete_input(&self) -> String {
        format!("hash: \"{}\"", self.hash())
    }
}

#[cfg(test)]
mod tests {
    use crate::common::block::Block;
    use crate::common::transaction::{ Transaction, TransactionAmount };

    #[test]
    fn block_hash_test() {
        let block = Block::new("hello_world".to_string(), 123456789, 420);
        assert_eq!(&"hello_world".to_string(), block.hash());
    }

    #[test]
    fn block_date_test() {
        let block = Block::new("hello_world".to_string(), 123456789, 420);
        assert_eq!(123456789, block.date());
    }

    #[test]
    fn block_height_test() {
        let block = Block::new("hello_world".to_string(), 123456789, 420);
        assert_eq!(420, block.height());
    }
    
    #[test]
    fn block_is_final_test() {
        let block = Block::new("hello_world".to_string(), 123456789, 420);
        assert_eq!(false, block.is_final());
    }
    
    #[test]
    fn block_set_transactions_test() {
        let mut block = Block::new("hello_world".to_string(), 123456789, 420);
        let tx = Transaction::new_from_block("hashy_transaction".to_string(), true, &block);
        
        block.set_transactions(vec![tx]);
        
        assert_eq!(1, block.transactions().len());
    }

    #[test]
    fn block_transaction_test() {
        let mut block = Block::new("hello_world".to_string(), 123456789, 420);
        let transaction = Transaction::new_from_block("hashy_transaction".to_string(), true, &block);
        
        let transacs = block.transactions_mut();
        transacs.push(transaction);

        assert_eq!(1, block.transactions().len());
    }

    // Test Deserialization and Serializations
    #[test]
    fn block_deserialize_test() {
        let data = r#"
        {
            "hash" : "blocky_hash",
            "time" : 123456789,
            "height" : 430690,
            "last_updated" : "2022-11-05T10:26:52.348613688Z",
            "is_final" : false,
            "transaction_collection" : [
                {
                    "hash": "hashy_hash", 
                    "date": 123456789,
                    "is_coinbase": false,
                    "block_hash" : "blocky_hash",
                    "block_height" : 430690,
                    "last_updated" : "2022-11-05T10:26:52.348613688Z",
                    "transaction_amount_collection" : [
                        {
                            "id": 5, 
                            "amount": 43.98,
                            "transaction_hash": "hashy_transaction",
                            "address_hash" : "hashy_address",
                            "index" : 42,
                            "date" : 123456789,
                            "vin_index" : -1,
                            "vin_hash" : null
                        },
                        {
                            "id": 5, 
                            "amount": 43.98,
                            "transaction_hash": "hashy_transaction",
                            "address_hash" : "hashy_address",
                            "index" : -1,
                            "date" : 123456789,
                            "vin_index" : 1,
                            "vin_hash" : null
                        },
                        {
                            "id": 5, 
                            "amount": 43.98,
                            "transaction_hash": "hashy_transaction",
                            "address_hash" : "hashy_address",
                            "index" : "-1",
                            "date" : 123456789,
                            "vin_index" : 1,
                            "vin_hash" : null
                        }
                    ]
                }
            ]
        }"#;

        // Parse the string of data into serde_json::Value.
        let block: Result<Block, serde_json::Error> = serde_json::from_str(data);
        if let Ok(b) = block {
            assert_eq!(&"blocky_hash".to_string(), b.hash());
            assert_eq!(123456789, b.date());
            assert_eq!(430690, b.height());
            assert_eq!(1, b.transactions().len());
        } else {
            println!("{:?}", block);
            assert!(false);
        }
    }
        // Test Deserialization and Serializations
    #[test]
    fn block_serialize_test() {
        
        let mut block = Block::new("blocky_hash".to_string(), 123456789, 430690);
        
        
        let mut transaction = Transaction::new_from_block("hashy_transaction".to_string(), true, &block);
        let transaction_amount = TransactionAmount::new(43.98, "hashy_address".to_string(), transaction.hash().clone(), 123456789, Some(42));
        
        let data = format!("{{\"hash\":\"blocky_hash\",\"date\":123456789,\"height\":430690,\"is_final\":false,\"last_updated\":\"{}\",\"transaction_collection\":[{{\"hash\":\"hashy_transaction\",\"date\":123456789,\"is_coinbase\":true,\"block_hash\":\"blocky_hash\",\"block_height\":430690,\"last_updated\":\"{}\",\"transaction_amount_collection\":[{{\"amount\":43.98,\"address_hash\":\"hashy_address\",\"transaction_hash\":\"hashy_transaction\",\"index\":42,\"date\":123456789,\"vin_index\":-1,\"vin_hash\":null}}]}}]}}", block.last_updated(), transaction.last_updated());
        
        let amounts = transaction.transaction_amounts_mut();
        amounts.push(transaction_amount);

        let transacs = block.transactions_mut();
        transacs.push(transaction);


        assert_eq!(data, serde_json::to_string(&block).unwrap())
    }
}