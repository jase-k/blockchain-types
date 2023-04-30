## PRE PARTITION

### BENCHMARKING SELECT

```sql get_addresses_needed_update
EXPLAIN (ANALYZE, BUFFERS, VERBOSE)
SELECT hash FROM address WHERE needs_update = true LIMIT 10000;
```

"QUERY PLAN"
"Limit  (cost=1000.00..3270870.40 rows=6165 width=37) (actual time=80941.978..81030.534 rows=1 loops=1)"
"  Output: hash"
"  Buffers: shared hit=1083781 read=1491657"
"  ->  Gather  (cost=1000.00..3270870.40 rows=6165 width=37) (actual time=80870.179..80958.733 rows=1 loops=1)"
"        Output: hash"
"        Workers Planned: 8"
"        Workers Launched: 8"
"        Buffers: shared hit=1083781 read=1491657"
"        ->  Parallel Seq Scan on public.address  (cost=0.00..3269253.90 rows=771 width=37) (actual time=71887.027..80859.018 rows=0 loops=9)"
"              Output: hash"
"              Filter: address.needs_update"
"              Rows Removed by Filter: 20487686"
"              Buffers: shared hit=1083781 read=1491657"
"              Worker 0:  actual time=80875.274..80875.275 rows=0 loops=1"
"                JIT:"
"                  Functions: 4"
"                  Options: Inlining true, Optimization true, Expressions true, Deforming true"
"                  Timing: Generation 0.172 ms, Inlining 19.031 ms, Optimization 5.874 ms, Emission 4.210 ms, Total 29.287 ms"
"                Buffers: shared hit=107814 read=168755"
"              Worker 1:  actual time=102.989..80850.903 rows=1 loops=1"
"                JIT:"
"                  Functions: 4"
"                  Options: Inlining true, Optimization true, Expressions true, Deforming true"
"                  Timing: Generation 0.178 ms, Inlining 19.563 ms, Optimization 6.020 ms, Emission 4.158 ms, Total 29.918 ms"
"                Buffers: shared hit=138024 read=148440"
"              Worker 2:  actual time=80859.396..80859.397 rows=0 loops=1"
"                JIT:"
"                  Functions: 4"
"                  Options: Inlining true, Optimization true, Expressions true, Deforming true"
"                  Timing: Generation 0.175 ms, Inlining 19.010 ms, Optimization 5.894 ms, Emission 4.190 ms, Total 29.269 ms"
"                Buffers: shared hit=103001 read=168159"
"              Worker 3:  actual time=80859.217..80859.217 rows=0 loops=1"
"                JIT:"
"                  Functions: 4"
"                  Options: Inlining true, Optimization true, Expressions true, Deforming true"
"                  Timing: Generation 0.183 ms, Inlining 19.642 ms, Optimization 5.763 ms, Emission 4.228 ms, Total 29.816 ms"
"                Buffers: shared hit=141436 read=159444"
"              Worker 4:  actual time=80852.311..80852.312 rows=0 loops=1"
"                JIT:"
"                  Functions: 4"
"                  Options: Inlining true, Optimization true, Expressions true, Deforming true"
"                  Timing: Generation 0.174 ms, Inlining 19.031 ms, Optimization 5.878 ms, Emission 4.205 ms, Total 29.288 ms"
"                Buffers: shared hit=106006 read=161962"
"              Worker 5:  actual time=80852.640..80852.640 rows=0 loops=1"
"                JIT:"
"                  Functions: 4"
"                  Options: Inlining true, Optimization true, Expressions true, Deforming true"
"                  Timing: Generation 0.178 ms, Inlining 19.681 ms, Optimization 5.765 ms, Emission 4.013 ms, Total 29.637 ms"
"                Buffers: shared hit=104597 read=179355"
"              Worker 6:  actual time=80859.404..80859.405 rows=0 loops=1"
"                JIT:"
"                  Functions: 4"
"                  Options: Inlining true, Optimization true, Expressions true, Deforming true"
"                  Timing: Generation 0.178 ms, Inlining 19.801 ms, Optimization 5.549 ms, Emission 3.732 ms, Total 29.260 ms"
"                Buffers: shared hit=140607 read=180310"
"              Worker 7:  actual time=80852.682..80852.682 rows=0 loops=1"
"                JIT:"
"                  Functions: 4"
"                  Options: Inlining true, Optimization true, Expressions true, Deforming true"
"                  Timing: Generation 0.188 ms, Inlining 20.131 ms, Optimization 5.868 ms, Emission 3.915 ms, Total 30.102 ms"
"                Buffers: shared hit=136673 read=155719"
"Planning:"
"  Buffers: shared hit=86"
"Planning Time: 0.291 ms"
"JIT:"
"  Functions: 37"
"  Options: Inlining true, Optimization true, Expressions true, Deforming true"
"  Timing: Generation 1.627 ms, Inlining 180.054 ms, Optimization 69.255 ms, Emission 56.967 ms, Total 307.903 ms"
"Execution Time: 81128.955 ms"


```sql get_last_parsed_block_height
SELECT height FROM block ORDER BY height DESC LIMIT 1;
```
"QUERY PLAN"
"Limit  (cost=0.43..0.48 rows=1 width=8) (actual time=3.611..3.612 rows=1 loops=1)"
"  Output: height"
"  Buffers: shared hit=4 read=1"
"  ->  Index Only Scan Backward using block_height_index on public.block  (cost=0.43..113626.34 rows=2461994 width=8) (actual time=3.610..3.611 rows=1 loops=1)"
"        Output: height"
"        Heap Fetches: 1"
"        Buffers: shared hit=4 read=1"
"Planning:"
"  Buffers: shared hit=72"
"Planning Time: 0.236 ms"
"Execution Time: 3.621 ms"


```sql update_address_coin_total
UPDATE address SET coin_total = (
            SELECT sum(amount) FROM transaction_amount WHERE address_hash = '{}'
        ), last_transaction = (
            SELECT date FROM transaction_amount WHERE address_hash = '{}' ORDER BY date desc LIMIT 1
        ), needs_update = false WHERE hash = '{}';
```
"QUERY PLAN"
"Update on public.address  (cost=39193.09..39201.13 rows=0 width=0) (actual time=0.977..0.977 rows=0 loops=1)"
"  Buffers: shared hit=3 read=2"
"  InitPlan 1 (returns $0)"
"    ->  Limit  (cost=19602.60..19602.60 rows=1 width=8) (never executed)"
"          Output: transaction_amount.date"
"          ->  Sort  (cost=19602.60..19615.44 rows=5135 width=8) (never executed)"
"                Output: transaction_amount.date"
"                Sort Key: transaction_amount.date DESC"
"                ->  Index Scan using fki_transaction_amounts_addresses on public.transaction_amount  (cost=0.70..19576.93 rows=5135 width=8) (never executed)"
"                      Output: transaction_amount.date"
"                      Index Cond: (transaction_amount.address_hash = '{}'::text)"
"  InitPlan 2 (returns $1)"
"    ->  Aggregate  (cost=19589.77..19589.80 rows=1 width=32) (never executed)"
"          Output: sum(transaction_amount_1.amount)"
"          ->  Index Scan using fki_transaction_amounts_addresses on public.transaction_amount transaction_amount_1  (cost=0.70..19576.93 rows=5135 width=6) (never executed)"
"                Output: transaction_amount_1.amount, transaction_amount_1.address_hash, transaction_amount_1.transaction_hash, transaction_amount_1.index, transaction_amount_1.date, transaction_amount_1.created_on, transaction_amount_1.vin_index, transaction_amount_1.vin_hash"
"                Index Cond: (transaction_amount_1.address_hash = '{}'::text)"
"  ->  Index Scan using address_hash on public.address  (cost=0.70..8.73 rows=1 width=47) (actual time=0.976..0.976 rows=0 loops=1)"
"        Output: $0, $1, false, address.ctid"
"        Index Cond: (address.hash = '{}'::text)"
"        Buffers: shared hit=3 read=2"
"Planning:"
"  Buffers: shared hit=146 read=8"
"Planning Time: 9.041 ms"
"Execution Time: 1.461 ms"



```sql get_address
SELECT hash, last_transaction, coin_total, is_miner, first_transaction, needs_update FROM address WHERE hash = $1";
```



```sql get_block
SELECT hash, height, date, last_updated, is_final FROM block WHERE height = $1", &[&height];
```



```sql get_transaction
SELECT hash, block_height, date, last_updated, block_hash, is_coinbase FROM transaction WHERE hash = $1", &[&transaction_hash];
```



```sql get_transaction_amount
SELECT transaction_hash, address_hash, amount, index, date, vin_index, vin_hash FROM transaction_amount WHERE transaction_hash = $1 AND index = $2 AND vin_index = $3", &[&inputs.0, &inputs.1, &inputs.2];
```



```sql get_coin_total_from_addresses
SELECT sum(coin_total) FROM address WHERE last_transaction > $1 AND needs_update = false", &[&date];
```



```sql get_total_active_addresses
SELECT count(hash) FROM address WHERE last_transaction > $1 AND needs_update = false", &[&date];
```



```sql get_block_from_date
SELECT * FROM block WHERE date > $1 ORDER BY HEIGHT asc LIMIT 1", &[&date];
```



```sql get_market_cap
SELECT sum(coin_total) FROM address WHERE needs_update = false;", &[];
```






