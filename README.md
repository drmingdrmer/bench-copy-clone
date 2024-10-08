# Bechmark Copy vs Clone in Rust

Run it:
```
make
```

The output would be like:

```text
make
cargo run --release -- --bench
   Compiling foo v0.1.0 (/Users/drdrxp/tt/2024-10-08-10-29-bench-copy-clone/foo)
    Finished `release` profile [optimized] target(s) in 0.71s
     Running `target/release/foo --bench`
copy                    time:   [286.40 ps 293.62 ps 300.71 ps]
                        change: [-3.2702% -1.4967% +0.3198%] (p = 0.12 > 0.05)
                        No change in performance detected.
Found 30 outliers among 100 measurements (30.00%)
  16 (16.00%) low severe
  1 (1.00%) low mild
  1 (1.00%) high mild
  12 (12.00%) high severe

clone                   time:   [298.71 ps 302.80 ps 306.79 ps]
                        change: [-0.5103% +1.6503% +3.7616%] (p = 0.13 > 0.05)
                        No change in performance detected.
Found 28 outliers among 100 measurements (28.00%)
  16 (16.00%) low severe
  2 (2.00%) low mild
  2 (2.00%) high mild
  8 (8.00%) high severe
```
