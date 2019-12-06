# Results
```
AOC 2019
Day 2 - Part 1 : 5866714
        generator: 4.9µs,
        runner: 19µs

Day 2 - Part 1 - rewrite : 5866714
        generator: 300ns,
        runner: 52.8µs

Day 2 - Part 2 : 5208    
        generator: 900ns,
        runner: 12.1064ms

Day 2 - Part 2 - clone_cpu : 5208
        generator: 2.9µs,
        runner: 2.2763ms

Day 2 - Part 2 - double_iter : 5208
        generator: 700ns,
        runner: 202.9µs

Day 2 - Part 2 - rewrite : 5208
        generator: 8.8µs,
        runner: 2.6891ms
```

# Benchmarks
```
Day2 - Part1/(default)  time:   [2.1298 us 2.1371 us 2.1449 us]
Found 6 outliers among 100 measurements (6.00%)
  5 (5.00%) high mild
  1 (1.00%) high severe
```
```
Day2 - Part1/rewrite    time:   [5.5009 us 5.5223 us 5.5447 us]
Found 7 outliers among 100 measurements (7.00%)
  7 (7.00%) high mild
```
```
Day2 - Part2/(default)  time:   [10.818 ms 10.834 ms 10.850 ms]
Found 4 outliers among 100 measurements (4.00%)
  3 (3.00%) high mild
  1 (1.00%) high severe
```
```
Day2 - Part2/clone_cpu  time:   [758.31 us 768.28 us 778.28 us]
Found 3 outliers among 100 measurements (3.00%)
  1 (1.00%) low mild
  2 (2.00%) high mild
```
```
Day2 - Part2/double_iter time:   [508.11 us 519.90 us 532.34 us]
Found 10 outliers among 100 measurements (10.00%)
  2 (2.00%) low severe
  4 (4.00%) low mild
  2 (2.00%) high mild
  2 (2.00%) high severe
```
```
Day2 - Part2/rewrite    time:   [1.2700 ms 1.2902 ms 1.3104 ms]
Found 8 outliers among 100 measurements (8.00%)
  1 (1.00%) low severe
  3 (3.00%) low mild
  3 (3.00%) high mild
  1 (1.00%) high severe
```