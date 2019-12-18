# Results
```
AOC 2019
Day 8 - Part 1 : 1485
        generator: 1.368µs,
        runner: 304.282µs

□□□■■□■■■■■□□■■□■■□■□□□□■
□■■□■□■■■■□■■□■□■□■■□■■■■
□■■□■□■■■■□■■□■□□■■■□□□■■
□□□■■□■■■■□□□□■□■□■■□■■■■
□■□■■□■■■■□■■□■□■□■■□■■■■
□■■□■□□□□■□■■□■□■■□■□■■■■

Day 8 - Part 2 : 0
        generator: 144ns,
        runner: 548.641µs
```

# Benchmarks
```
Day8 - Part1/(default)  time:   [137.24 us 137.86 us 138.53 us]
                        change: [-1.9571% -1.1640% -0.3028%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 10 outliers among 100 measurements (10.00%)
  7 (7.00%) high mild
  3 (3.00%) high severe
```

This benchmark is heavily skewed by printing
```
Day8 - Part2/(default)  time:   [3.4263 ms 3.4694 ms 3.5125 ms]
Found 9 outliers among 100 measurements (9.00%)
  1 (1.00%) low severe
  2 (2.00%) low mild
  4 (4.00%) high mild
  2 (2.00%) high severe
```