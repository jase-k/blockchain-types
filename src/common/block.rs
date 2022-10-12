use serde::{Deserialize, Serialize};
use named_type_derive::*;
use named_type::NamedType;
use devii::devii::FetchFields;
use getset::{CopyGetters, Getters};

use crate::common::transaction::{Transaction};

#[derive(Serialize, Deserialize, Debug, Clone, NamedType, Default, Getters, CopyGetters)]
pub struct Block {
    #[getset(get = "pub")]
    hash: String, // Primary Key
    
    #[getset(get_copy = "pub")]
    date: u64,

    #[getset(get_copy = "pub")]
    height: u64,
    
    #[getset(get, get_mut)]
    transactions: Vec<Transaction>
}

impl Block {
    pub fn new(hash: String, date: u64, height: u64) -> Self {
        Block {
            hash,
            date, 
            height, 
            transactions: vec![]
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::common::block::Block;
    use crate::common::transaction::Transaction;

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

    // #[test]
    // fn block_transaction_test() {
    //     let mut block = Block::new("hello_world".to_string(), 123456789, 420);
    //     let transaction = Transaction::new();
    //     let transacs = block.transactions_mut()
    //     transacs
    //     assert_eq!(420, block.height());
    // }

    // Test Deserialization and Serializations
}