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
