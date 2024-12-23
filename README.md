# Advent of Code 2024

![Build Status](https://github.com/chriswmann/aoc2024/actions/workflows/rust.yml/badge.svg?branch=main) [![License: Apache-2.0](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0) [![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

AoC 2024 in rust.

To run a specific day, use `cargo r --bin <DAY>`, where `DAY` is like `day01`.
Note: Day 3 uses the regex crate's `pattern` feature, which requires
nightly rust. Day 3 can therefore by run using `cargo +nightly r --bin day03`.

Or run `just run-day <DAY>`, e.g. `just run-day 03` (zero padding is required).

To run unit tests across the workspace use `cargo t --workspace --exclude day05`.
Or run `just run-tests`.

To add a new day, run `just add-day <DAY>`, again with `DAY` a zero-padded
two-digit number.
