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
