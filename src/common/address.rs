use serde::{Deserialize, Serialize};
// use serde::ser::{Serialize, SerializeStruct, Serializer};
use named_type_derive::*;
use named_type::NamedType;
// use devii::devii::FetchFields;
use getset::{CopyGetters, Getters, MutGetters, Setters};


use crate::common::transaction::{TransactionAmount};

#[allow(dead_code)]
#[derive(Deserialize, Debug, Clone, NamedType, Default, Getters, CopyGetters)]
pub struct Address {
    #[getset(get = "pub")]
    hash: String, // Primary Key 5 bytes
    
    #[getset(get_copy = "pub")]
    last_transaction: u64, // 4 bytes
    
    #[getset(get_copy = "pub")]
    coin_total: f64, // 8 bytes 
    
    #[getset(get = "pub")]
    file_url: Option<String>, // 7 bytes 
    
    #[getset(get_copy = "pub")]
    is_miner: bool, // 1 bit
    
    // Stored in Blob Storage
    #[getset(get_copy = "pub")]
    first_transaction: u32,
    
    #[getset(get = "pub")]
    transactions: Vec<TransactionAmount> 
}

impl Address {
    pub fn new(hash: String) -> Self {
        Address {   
            hash, 
            last_transaction: 0, 
            coin_total: 0.0, 
            file_url: None, 
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
}

