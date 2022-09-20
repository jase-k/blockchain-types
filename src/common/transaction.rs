#[allow(dead_code)]
pub struct Transaction {
    date: u32,
    is_coinbase: bool, 
    hash: String,
    block_hash: String,
    block_height: u64,
    amounts: Option<Vec<TransactionAmount>>
}

#[allow(dead_code)]
pub struct TransactionAmount {
    amount: u64,
    address: String, 
    transaction_hash: String
}