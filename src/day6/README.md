# Results

```
AOC 2019
Day 6 - Part 1 : 224901
        generator: 501µs,
        runner: 707.9µs

Day 6 - Part 2 : 334
        generator: 454µs,
        runner: 46.0499ms

Day 6 - Part 2 - intersect : 334
        generator: 505.6µs,
        runner: 573.1µs

Day 6 - Part 2 - intersect_hashmap : 334
        generator: 503.9µs,
        runner: 432.4µs
```

# Benchmarks

```
Day6 - Part1/(default)  time:   [512.30 us 514.60 us 517.19 us]
                        change: [-1.5499% -0.2915% +0.9906%] (p = 0.68 > 0.05)
                        No change in performance detected.
Found 7 outliers among 100 measurements (7.00%)
  3 (3.00%) high mild
  4 (4.00%) high severe
```
```
Day6 - Part2/(default)  time:   [33.202 ms 33.224 ms 33.247 ms]
                        change: [-1.0173% -0.8389% -0.6573%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 9 outliers among 100 measurements (9.00%)
  5 (5.00%) low mild
  3 (3.00%) high mild
  1 (1.00%) high severe
```
```
Day6 - Part2/intersect  time:   [416.99 us 418.91 us 421.11 us]
                        change: [-34.636% -34.166% -33.648%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 8 outliers among 100 measurements (8.00%)
  3 (3.00%) high mild
  5 (5.00%) high severe
```
```
Day6 - Part2/intersect_hashmap
                        time:   [378.43 us 379.89 us 381.49 us]
Found 7 outliers among 100 measurements (7.00%)
  6 (6.00%) high mild
  1 (1.00%) high severe
```

# Notes
The initial implementation I had for this path traversal was absolutely terrible. For some reason I couldn't remember that all planets lead towards COM and you can very easily utilize that for pathfinding. Instead my original pathfinding would work on a universe that was not centralized, by spawning threads for each possible path at each interesection. It worked, but it is extremely complicated compared to finding both paths to COM, and adding the distance at the intersection for both.