![tests](https://github.com/cadiac/adventofcode/actions/workflows/tests.yml/badge.svg)

# 🎄🎄🎄 Advent of Code 2022 🎄🎄🎄

Solutions to [Advent of Code 2022](https://adventofcode.com/) implemented in [Rust](https://www.rust-lang.org).

## Installing

Follow [Rust](https://www.rust-lang.org/en-US/install.html) installation instructions.

## Running solutions

The project is split into separate binaries within one cargo project.

You can run individual solutions with

```bash
$ cargo run -- --day day01 --file ./inputs/day01.txt
```

or run single day's tests with

```bash
$ cargo test --lib -- solution::day01::tests
```

within the project's root directory

## Solutions

❄️ [Day 01](src/bin/day01.rs)
