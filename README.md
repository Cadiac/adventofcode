![tests](https://github.com/cadiac/adventofcode/actions/workflows/tests.yml/badge.svg)

# 🎄🎄🎄 Advent of Code 2022 🎄🎄🎄

Solutions to [Advent of Code 2022](https://adventofcode.com/) implemented in [Rust](https://www.rust-lang.org).

Online solutions runner: https://aoc.cadi.ac/

## Installing

Follow [Rust](https://www.rust-lang.org/en-US/install.html) installation instructions.

## Running the solutions

The project is split into separate binaries within one cargo project.

You can run individual solutions using custom inputs with

```bash
$ cargo run -- --day 1 --file ./inputs/day01.txt
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

## Running the web project

This repository now also ships with a small web tool, [https://aoc.cadi.ac/](https://aoc.cadi.ac/) to run the solutions online.

To get started with local web development environment start by adding `wasm32-unknown-unknown` toolchain:

```bash
$ rustup target add wasm32-unknown-unknown
```

Then install trunk

```bash
$ cargo install --locked trunk
```

and start the local development server with

```bash
$ trunk serve
```

This should start the server at `localhost:8080`.

## Solutions

❄️ [Day 01](src/solution/day01.rs)
❄️ [Day 02](src/solution/day02.rs)
❄️ [Day 03](src/solution/day03.rs)
❄️ [Day 04](src/solution/day04.rs)
❄️ [Day 05](src/solution/day05.rs)
❄️ [Day 06](src/solution/day06.rs)
❄️ [Day 07](src/solution/day07.rs)
❄️ [Day 08](src/solution/day08.rs)
❄️ [Day 09](src/solution/day09.rs)
❄️ [Day 10](src/solution/day10.rs)
❄️ [Day 11](src/solution/day11.rs)
❄️ [Day 12](src/solution/day12.rs)
❄️ [Day 13](src/solution/day13.rs)
❄️ [Day 14](src/solution/day14.rs)
❄️ [Day 15](src/solution/day15.rs)
❄️ [Day 16](src/solution/day16.rs)
❄️ [Day 17](src/solution/day17.rs)
❄️ [Day 18](src/solution/day18.rs)
x [Day 19](src/solution/day19.rs)
❄️ [Day 20](src/solution/day20.rs)
