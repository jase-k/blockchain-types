use serde::{Deserialize, Serialize};
// use serde::ser::{Serialize, SerializeStruct, Serializer};
use named_type_derive::*;
use named_type::NamedType;
// use devii::devii::FetchFields;
use getset::{CopyGetters, Getters};


use crate::common::transaction::{TransactionAmount};

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug, Clone, NamedType, Default, Getters, CopyGetters)]
pub struct Address {
    #[getset(get = "pub")]
    hash: String, // Primary Key 5 bytes
    
    #[getset(get_copy = "pub")]
    last_transaction: u64, // 4 bytes
    
    #[getset(get_copy = "pub")]
    coin_total: f64, // 8 bytes 
    
    #[getset(get_copy = "pub")]
    is_miner: bool, // 1 bit
    
    // Stored in Blob Storage
    #[getset(get_copy = "pub")]
    first_transaction: u64,
    
    #[getset(get = "pub")]
    transactions: Vec<TransactionAmount> 
}

impl Address {
    pub fn new(hash: String) -> Self {
        Address {   
            hash, 
            last_transaction: 0, 
            coin_total: 0.0, 
            is_miner: false,
            first_transaction: 0,
            transactions: Vec::new() 
        }
    }

    pub fn add_transaction_amount(&mut self, tx_amount: TransactionAmount, is_miner: bool) ->  &mut Self {
        if is_miner == true {
            self.is_miner = true;
        }

        self.coin_total += tx_amount.amount();

        if self.transactions.len() == 0 {
            self.first_transaction = tx_amount.date();
        }
        
        self.transactions.push(tx_amount);

        self.last_transaction = self.get_latest_transaction();

        self
    }

    // Private Methods:
    fn get_latest_transaction(&mut self) -> u64 {
        if self.transactions.len() < 1 {
            return 0 
        } else {
            self.transactions.sort_by(|a, b| b.date().cmp(&a.date()));
            let mut iter = self.transactions.iter();
            return iter.next().unwrap().date();
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::common::address::Address;
    use crate::common::transaction::TransactionAmount;
    

    #[test]
    fn add_transaction_amount_test() {
        let mut address = Address::new("hashy_address".to_string());
        let transaction_amount = TransactionAmount::new(90.8, "address".to_string(), "transaction_hash".to_string(), 123456789, Some(5));

        address.add_transaction_amount(transaction_amount, false); 

        assert_eq!(address.last_transaction(), 123456789);
    }
    
    #[test]
    fn add_two_transaction_amount_test() {
        let mut address = Address::new("hashy_address".to_string());
        let transaction_amount = TransactionAmount::new(90.8, "address".to_string(), "transaction_hash".to_string(), 123456789, Some(5));
        let transaction_amount2 = TransactionAmount::new(90.8, "address".to_string(), "transaction_hash".to_string(), 987654321, Some(5));


        address.add_transaction_amount(transaction_amount, false); 
        address.add_transaction_amount(transaction_amount2, true); 

        assert_eq!(address.last_transaction(), 987654321);
    }
    
    #[test]
    fn deserialize_test() {
        let raw = r#"{
            "hash":"hashy_address",
            "last_transaction":123456789,
            "coin_total":10.0,
            "is_miner":true,
            "first_transaction":111156789,
            "transactions":[]
        }"#;
        let address: Result<Address, serde_json::Error> = serde_json::from_str(raw);
        if let Ok(a) = address {
            assert_eq!(a.hash().clone(), "hashy_address".to_string());
            assert_eq!(a.last_transaction(), 123456789);
            assert_eq!(a.first_transaction(), 111156789);
            assert_eq!(a.coin_total(), 10.0);
            assert_eq!(a.is_miner(), true);
            assert_eq!(a.transactions(), &vec![]);
        } else {
            println!("{:?}", address);
            assert!(false)
        }
    }

    #[test]
    fn deserialize_with_transactions_test() {
        let raw = r#"{
            "hash":"hashy_address",
            "last_transaction":123456789,
            "coin_total":10.0,
            "is_miner":true,
            "first_transaction":111156789,
            "transactions":[
                {
                    "id": 5, 
                    "amount": 43.98,
                    "transaction_hash": "hashy_transaction",
                    "address" : "hashy_address",
                    "index" : 42,
                    "date" : 111456789
                },
                {
                    "amount": 3.8,
                    "transaction_hash": "hashy_transaction2",
                    "address" : "hashy_address",
                    "index" : 42,
                    "date" : 123456789
                }
            ]
        }"#;
        let address: Result<Address, serde_json::Error> = serde_json::from_str(raw);
        if let Ok(a) = address {
            assert_eq!(a.transactions().len(), 2)
        } else {
            println!("{:?}", address);
            assert!(false)
        }
    }

    #[test]
    fn serialize_test() {
        let raw = r#"{"hash":"hashy_address","last_transaction":0,"coin_total":0.0,"is_miner":false,"first_transaction":0,"transactions":[]}"#;
        
        let address = Address::new("hashy_address".to_string());

        let result = serde_json::to_string(&address);

        if let Ok(res) = result {
            assert_eq!(raw, res)
        } else {
            println!("{:?}", result);
            assert!(false)
        }      
    }

    #[test]
    fn serialize_with_transaction_test() {
        let raw = r#"{"hash":"hashy_address","last_transaction":123456789,"coin_total":43.98,"is_miner":true,"first_transaction":123456789,"transactions":[{"amount":43.98,"address":"hashy_address","transaction_hash":"hashy_transaction","index":4,"date":123456789}]}"#;

        let tx_amount = TransactionAmount::new(43.98, "hashy_address".to_string(), "hashy_transaction".to_string(), 123456789, Some(4));
        
        let mut address = Address::new("hashy_address".to_string());
        address.add_transaction_amount(tx_amount, true);
        
        let result = serde_json::to_string(&address);

        if let Ok(res) = result {
            assert_eq!(raw, res)
        } else {
            println!("{:?}", result);
            assert!(false)
        }     
    }
}

// Address {   
//     hash, 
//     last_transaction: 0, 
//     coin_total: 0.0, 
//     file_url: None, 
//     is_miner: false,
//     first_transaction: 0,
//     transactions: Vec::new() 
// }