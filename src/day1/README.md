# Results
```
AOC 2019
Day 1 - Part 1 - pure_iter : 3402634
        generator: 18.4µs,
        runner: 400ns

Day 1 - Part 1 - rayon_iter : 3402634
        generator: 6µs,
        runner: 690.7µs

Day 1 - Part 2 - pure_iter_2 : 5101069
        generator: 7.5µs,
        runner: 2µs

Day 1 - Part 2 - rayon_iter_2 : 5101069
        generator: 6.4µs,
        runner: 135µs
```
# Benchmarks
```
Day1 - Part1/pure_iter  time:   [58.029 ns 58.425 ns 58.911 ns]
                        change: [-0.9190% +0.1387% +1.2397%] (p = 0.81 > 0.05)
                        No change in performance detected.
Found 13 outliers among 100 measurements (13.00%)
  5 (5.00%) high mild
  8 (8.00%) high severe
Day1 - Part1/rayon_iter time:   [15.754 us 15.837 us 15.935 us]
                        change: [-3.6743% -2.6033% -1.6030%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 8 outliers among 100 measurements (8.00%)
  6 (6.00%) high mild
  2 (2.00%) high severe
```
```
Day1 - Part2/pure_iter_2
                        time:   [1.1143 us 1.1182 us 1.1225 us]
                        change: [-13.466% -12.711% -12.029%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 8 outliers among 100 measurements (8.00%)
  4 (4.00%) high mild
  4 (4.00%) high severe
```
```
Day1 - Part2/rayon_iter_2
                        time:   [16.591 us 16.697 us 16.814 us]
                        change: [-0.9698% +0.0054% +0.9308%] (p = 0.99 > 0.05)
                        No change in performance detected.
Found 7 outliers among 100 measurements (7.00%)
  6 (6.00%) high mild
  1 (1.00%) high severe
```
```
Generator Day1/pure_iter
                        time:   [3.0013 us 3.0332 us 3.0688 us]
Found 10 outliers among 100 measurements (10.00%)
  6 (6.00%) high mild
  4 (4.00%) high severe
Generator Day1/rayon_iter
                        time:   [3.0264 us 3.0543 us 3.0874 us]
Found 6 outliers among 100 measurements (6.00%)
  3 (3.00%) high mild
  3 (3.00%) high severe
```
```
Generator Day1/pure_iter_2
                        time:   [3.0587 us 3.0873 us 3.1191 us]
Found 7 outliers among 100 measurements (7.00%)
  2 (2.00%) high mild
  5 (5.00%) high severe
Generator Day1/rayon_iter_2
                        time:   [3.0019 us 3.0350 us 3.0756 us]
Found 11 outliers among 100 measurements (11.00%)
  5 (5.00%) high mild
  6 (6.00%) high severe