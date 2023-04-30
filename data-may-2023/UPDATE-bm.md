## PRE PARTITION

### BENCHMARKING UPDATE

```sql insert_address
INSERT INTO address (hash, last_updated, needs_update) 
        VALUES ($1, Now(), True) 
        ON CONFLICT ON CONSTRAINT addresses_pkey DO UPDATE SET last_updated=now(), needs_update=true;",
        &[hash];
```


```sql update_address_coin_total
UPDATE address SET coin_total = (
            SELECT sum(amount) FROM transaction_amount WHERE address_hash = '{}'
        ), last_transaction = (
            SELECT date FROM transaction_amount WHERE address_hash = '{}' ORDER BY date desc LIMIT 1
        ), needs_update = false WHERE hash = '{}';", address_hash, address_hash, address_hash).as_str(),
        &[];
```


```sql get_addresses_needed_update
SELECT hash FROM address WHERE needs_update = true LIMIT $1;
```


```sql get_address
SELECT hash, last_transaction, coin_total, is_miner, first_transaction, needs_update FROM address WHERE hash = $1", &[address_hash];
```


```sql insert_block
INSERT INTO block (hash, height, date, last_updated, is_final) VALUES ($1, $2, $3, Now(), $4)",
&[block.hash(), &block.height(), &block.date(), &block.is_final()];
```


