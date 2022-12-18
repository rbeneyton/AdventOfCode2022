# Advent Of Code 2022

https://adventofcode.com/2022 solutions in Rust.

Retrieve your daily input using your session cookie via:
```sh
cargo run --release -- --day <day> download --session <session>
```
The data is put in data/ and used directly at compile time.

To compute the  execution time, use:
```sh
cargo run --release -- --day <day> execute --part <part>
```

To measure execution time for a particular day, use:
```sh
cargo run --release -- --day <day> benchmark --number <number> --current
```

## [Day 01: Calorie Counting](https://adventofcode.com/2022/day/1)

double iterator::fold() can be used too, but…
[Code](./src/solutions/day01.rs)

## [Day 02: Rock Paper Scissors](https://adventofcode.com/2022/day/2)

basic logic computation.

Sadly inpt crate is slow (>500µs to parse 2500 inputs), so went back to
"manual" parsing.
[Code](./src/solutions/day02.rs)

## [Day 03: Rucksack Reorganization](https://adventofcode.com/2022/day/3)

rust iterators playground…
[Code](./src/solutions/day03.rs)

## [Day 04: Camp Cleanup](https://adventofcode.com/2022/day/4)

range comparisons.
[Code](./src/solutions/day04.rs)

## [Day 05: Supply Stacks](https://adventofcode.com/2022/day/5)

parsing harder than task.
task 2 is in quick and dirty coded task 1.
[Code](./src/solutions/day05.rs)

## [Day 06: Tuning Trouble](https://adventofcode.com/2022/day/6)

scanning with 'appropriate' memory.
naive code would have been:
```rust
use itertools::Itertools;

Solution::U64(
    input
        .chars()
        .tuple_windows()
        .enumerate()
        .skip_while(|(idx, (a, b, c, d))| ![a, b, c, d].iter().all_unique())
        .map(|(idx, _)| (idx + 4) as u64)
        .next()
        .unwrap())
```
which is 145µs for simple 4 wide deep scan, but mine is 24µs.
Difference with 14 deep would have been huge of course…
[Code](./src/solutions/day06.rs)

## [Day 07: No Space Left On Device](https://adventofcode.com/2022/day/7)

Trees in rust are quite annoying to manipulate, so here is a basic hashmap based
solution. Sadly this leads to too many string manipulations, we need a way to
avoid them.
Each part uses 165µs.

update: new version using tree, and without any String, reduced to 46µs.
[Code](./src/solutions/day07.rs)


## [Day 08: Treetop Tree House](https://adventofcode.com/2022/day/8)

inplace bitmask in a grid, to save as space as possible.

[Code](./src/solutions/day08.rs)

## [Day 09: Rope Bridge](https://adventofcode.com/2022/day/9)

basic movement computations.

[Code](./src/solutions/day09.rs)

## [Day 10: Cathode-Ray Tube](https://adventofcode.com/2022/day/10)

CPU register simulation, nothing special.

[Code](./src/solutions/day10.rs)

## [Day 11: Monkey in the Middle](https://adventofcode.com/2022/day/11)

Unkown game, plenty of subtle unsaid conditions, weird parsing for no reason, …
I should have done the atcoder.jp game.
Solution weren't prime factor decomposition as I first guessed, but simple lowering value by detecting invariant!

[Code](./src/solutions/day11.rs)

## [Day 12: Hill Climbing Algorithm](https://adventofcode.com/2022/day/12)

path searching, basic Dijkstra

[Code](./src/solutions/day12.rs)

## [Day 13: Distress Signal](https://adventofcode.com/2022/day/13)

parsing possible without recursion

[Code](./src/solutions/day13.rs)
