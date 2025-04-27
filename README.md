# Custom Hashers

```sh
cargo bench
```

```sh
u64 hashing/tiny_murmur/0
                        time:   [836.92 ps 838.92 ps 842.01 ps]
                        change: [-21.015% -19.703% -18.586%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 12 outliers among 100 measurements (12.00%)
  1 (1.00%) high mild
  11 (11.00%) high severe
u64 hashing/DefaultHasher/0
                        time:   [3.8463 ns 3.8551 ns 3.8666 ns]
                        change: [-25.403% -24.069% -22.628%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 16 outliers among 100 measurements (16.00%)
  1 (1.00%) low mild
  15 (15.00%) high severe
u64 hashing/tiny_murmur/1
                        time:   [839.01 ps 840.09 ps 841.55 ps]
                        change: [-19.702% -18.564% -17.246%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 14 outliers among 100 measurements (14.00%)
  1 (1.00%) high mild
  13 (13.00%) high severe
u64 hashing/DefaultHasher/1
                        time:   [3.7837 ns 3.7882 ns 3.7967 ns]
                        change: [-26.554% -25.195% -24.021%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 13 outliers among 100 measurements (13.00%)
  1 (1.00%) high mild
  12 (12.00%) high severe
u64 hashing/tiny_murmur/6148914691236517205
                        time:   [851.05 ps 852.83 ps 855.55 ps]
                        change: [-18.782% -17.435% -15.902%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 16 outliers among 100 measurements (16.00%)
  1 (1.00%) high mild
  15 (15.00%) high severe
u64 hashing/DefaultHasher/6148914691236517205
                        time:   [3.8394 ns 3.8427 ns 3.8479 ns]
                        change: [-26.526% -25.283% -24.058%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 14 outliers among 100 measurements (14.00%)
  1 (1.00%) high mild
  13 (13.00%) high severe
u64 hashing/tiny_murmur/16045690984833335023
                        time:   [838.13 ps 839.92 ps 842.66 ps]
                        change: [-21.000% -19.713% -18.266%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 11 outliers among 100 measurements (11.00%)
  1 (1.00%) high mild
  10 (10.00%) high severe
u64 hashing/DefaultHasher/16045690984833335023
                        time:   [3.8402 ns 3.8436 ns 3.8489 ns]
                        change: [-28.031% -26.678% -25.381%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 14 outliers among 100 measurements (14.00%)
  1 (1.00%) high mild
  13 (13.00%) high severe
u64 hashing/tiny_murmur/123456789
                        time:   [851.02 ps 852.37 ps 854.39 ps]
                        change: [-22.959% -21.922% -20.736%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 18 outliers among 100 measurements (18.00%)
  1 (1.00%) high mild
  17 (17.00%) high severe
u64 hashing/DefaultHasher/123456789
                        time:   [3.8430 ns 3.8524 ns 3.8646 ns]
                        change: [-27.975% -26.992% -25.808%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 16 outliers among 100 measurements (16.00%)
  1 (1.00%) high mild
  15 (15.00%) high severe
u64 hashing/tiny_murmur/4294967295
                        time:   [837.59 ps 838.42 ps 839.64 ps]
                        change: [-25.103% -23.762% -22.399%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 14 outliers among 100 measurements (14.00%)
  3 (3.00%) high mild
  11 (11.00%) high severe
u64 hashing/DefaultHasher/4294967295
                        time:   [3.7823 ns 3.7868 ns 3.7931 ns]
                        change: [-31.962% -31.221% -30.228%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 10 outliers among 100 measurements (10.00%)
  10 (10.00%) high severe
u64 hashing/tiny_murmur/18446744073709551615
                        time:   [838.64 ps 842.58 ps 848.19 ps]
                        change: [-25.631% -24.904% -23.766%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 13 outliers among 100 measurements (13.00%)
  13 (13.00%) high severe
u64 hashing/DefaultHasher/18446744073709551615
                        time:   [3.8438 ns 3.8477 ns 3.8555 ns]
                        change: [-30.732% -30.124% -29.445%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 11 outliers among 100 measurements (11.00%)
  11 (11.00%) high severe
```