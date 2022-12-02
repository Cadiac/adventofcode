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

or all solutions using defaults inputs from `inputs/` directory with

```bash
$ cargo run
```

Single day's tests can be run with

```bash
$ cargo test --lib -- solution::day01::tests
```

or all tests with

```bash
$ cargo test
```

within the project's root directory.

## Solutions

❄️ [Day 01](src/bin/day01.rs)
❄️ [Day 02](src/bin/day02.rs)