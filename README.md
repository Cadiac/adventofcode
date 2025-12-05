[![Tests and Deploy](https://github.com/Cadiac/adventofcode/actions/workflows/deploy.yml/badge.svg)](https://github.com/Cadiac/adventofcode/actions/workflows/deploy.yml)

# 🎄🎄🎄 Advent of Code 2025 🎄🎄🎄

Solutions to [Advent of Code 2025](https://adventofcode.com/) implemented in [Rust](https://www.rust-lang.org).

Online solutions runner: https://aoc.cadi.ac/

## Installing

Follow [Rust](https://www.rust-lang.org/en-US/install.html) installation instructions.

## Running the solutions

The project is split into separate binaries within one cargo project.

You can run individual solutions with

```bash
$ cargo run -- --day 1 --year 2025
```

or using custom inputs with

```bash
$ cargo run -- --day 1 --year 2025 --file inputs/2025/day01.txt
```

All solutions of the current year can be run using defaults inputs from `inputs/` directory with

```bash
$ cargo run
```

Single day's tests can be run with

```bash
$ cargo test --workspace -- y2025::day01::tests
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

❄️ [Day 01](aoc-solver/src/y2025/day01.rs)
❄️ [Day 02](aoc-solver/src/y2025/day02.rs)
❄️ [Day 03](aoc-solver/src/y2025/day03.rs)
❄️ [Day 04](aoc-solver/src/y2025/day04.rs)
❄️ [Day 05](aoc-solver/src/y2025/day05.rs)
