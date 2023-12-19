[![Tests and Deploy](https://github.com/Cadiac/adventofcode/actions/workflows/deploy.yml/badge.svg)](https://github.com/Cadiac/adventofcode/actions/workflows/deploy.yml)

# 🎄🎄🎄 Advent of Code 2023 🎄🎄🎄

Solutions to [Advent of Code 2023](https://adventofcode.com/) implemented in [Rust](https://www.rust-lang.org).

Online solutions runner: https://aoc.cadi.ac/

## Installing

Follow [Rust](https://www.rust-lang.org/en-US/install.html) installation instructions.

## Running the solutions

The project is split into separate binaries within one cargo project.

You can run individual solutions with

```bash
$ cargo run -- --day 1 --year 2023
```

or using custom inputs with

```bash
$ cargo run -- --day 1 --year 2023 --file inputs/2023/day01.txt
```

All solutions of the current year can be run using defaults inputs from `inputs/` directory with

```bash
$ cargo run
```

Single day's tests can be run with

```bash
$ cargo test --workspace -- y2023::day01::tests
```

or all tests with

```bash
$ cargo test --workspace
```

within the project's root directory.

## Benchmarking

You can run all benchmarks within `aoc-solver` directory with

```bash
$ cargo bench
```

Individual day benchmarks can be run using

```bash
cargo bench -- day-1 --exact
```

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

and within the `aoc-web` directory start the local development server with

```bash
$ trunk serve
```

This should start the server at `localhost:8080`.

## Solutions

❄️ [Day 01](aoc-solver/src/y2023/day01.rs)
❄️ [Day 02](aoc-solver/src/y2023/day02.rs)
❄️ [Day 03](aoc-solver/src/y2023/day03.rs)
❄️ [Day 04](aoc-solver/src/y2023/day04.rs)
❄️ [Day 05](aoc-solver/src/y2023/day05.rs)
❄️ [Day 06](aoc-solver/src/y2023/day06.rs)
❄️ [Day 07](aoc-solver/src/y2023/day07.rs)
❄️ [Day 08](aoc-solver/src/y2023/day08.rs)
❄️ [Day 09](aoc-solver/src/y2023/day09.rs)
❄️ [Day 10](aoc-solver/src/y2023/day10.rs)
❄️ [Day 11](aoc-solver/src/y2023/day11.rs)
❄️ [Day 12](aoc-solver/src/y2023/day12.rs)
❄️ [Day 13](aoc-solver/src/y2023/day13.rs)
❄️ [Day 14](aoc-solver/src/y2023/day14.rs)
❄️ [Day 15](aoc-solver/src/y2023/day15.rs)
❄️ [Day 16](aoc-solver/src/y2023/day16.rs)
❄️ [Day 17](aoc-solver/src/y2023/day17.rs)
❄️ [Day 18](aoc-solver/src/y2023/day18.rs)
❄️ [Day 19](aoc-solver/src/y2023/day19.rs)
