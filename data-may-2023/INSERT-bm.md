## PRE PARTITION

### BENCHMARKING INSERT

```sql insert_address
INSERT INTO address (hash, last_updated, needs_update) 
        VALUES ($1, Now(), True) 
        ON CONFLICT ON CONSTRAINT addresses_pkey DO UPDATE SET last_updated=now(), needs_update=true;",
        &[hash];
```


```sql insert_block
INSERT INTO block (hash, height, date, last_updated, is_final) VALUES ($1, $2, $3, Now(), $4)",
        &[block.hash(), &block.height(), &block.date(), &block.is_final()],
```


```sql insert_transaction
INSERT INTO transaction (hash, block_height, date, last_updated, block_hash, is_coinbase) 
        VALUES ($1, $2, $3, Now(), $4, $5)",
        &[transaction.hash(), &transaction.block_height(), &transaction.date(), &transaction.block_hash(), &transaction.is_coinbase()],
```


```sql insert_transactions
INSERT INTO transaction (hash, block_height, date, last_updated, block_hash, is_coinbase) 
        VALUES {}", &values.join(",") ).as_str(),
        &[],
```


```sql insert_transaction_amount
INSERT INTO transaction_amount (transaction_hash, address_hash, amount, date, index, vin_index) VALUES ($1, $2, $3, $4, $5, $6)",
            &[transaction_amount.transaction_hash(), transaction_amount.address_hash(), &Decimal::from_f64(transaction_amount.amount()), &transaction_amount.date(), &transaction_amount.index(), &transaction_amount.vin_index()],
```
```sql 
INSERT INTO transaction_amount (transaction_hash, address_hash, amount, date, index, vin_index, vin_hash) VALUES ($1, $2, $3, $4, $5, $6, $7)",
                &[transaction_amount.transaction_hash(), transaction_amount.address_hash(), &Decimal::from_f64(transaction_amount.amount()), &transaction_amount.date(), &transaction_amount.index(), &transaction_amount.vin_index(), &vin_hash],
```


```sql insert_transaction_amounts
INSERT INTO transaction_amount (transaction_hash, address_hash, amount, date, index, vin_index, vin_hash) VALUES {}", values.join(",")).as_str(),
        &[],
```




















```sql insert_transaction
INSERT INTO transaction (hash, block_height, date, last_updated, block_hash, is_coinbase) 
VALUES ($1, $2, $3, Now(), $4, $5)",
            &[transaction.hash(), &transaction.block_height(), &transaction.date(), &transaction.block_hash(), &transaction.is_coinbase()];
```

```sql insert_transactions
INSERT INTO transaction (hash, block_height, date, last_updated, block_hash, is_coinbase) 
VALUES {}", &values.join(",") ).as_str(), &[];
```

```sql insert_transaction_amount
INSERT INTO transaction_amount (transaction_hash, address_hash, amount, date, index, vin_index) VALUES ($1, $2, $3, $4, $5, $6)",
            &[transaction_amount.transaction_hash(), transaction_amount.address_hash(), &Decimal::from_f64(transaction_amount.amount()), &transaction_amount.date(), &transaction_amount.index(), &transaction_amount.vin_index()];
            ```

 INSERT INTO transaction_amount (transaction_hash, address_hash, amount, date, index, vin_index, vin_hash) VALUES ($1, $2, $3, $4, $5, $6, $7)",
                &[transaction_amount.transaction_hash(), transaction_amount.address_hash(), &Decimal::from_f64(transaction_amount.amount()), &transaction_amount.date(), &transaction_amount.index(), &transaction_amount.vin_index(), &vin_hash];



