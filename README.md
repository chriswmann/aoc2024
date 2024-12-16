# Advent of Code 2024

AoC 2024 in rust.

To run a specific day, use `cargo r --bin <DAY>`, where `DAY` is like `day01`.

Note: Day 3 uses the regex crate's `pattern` feature, which requires
nightly rust. Day 3 can therefore by run using `cargo +nightly r --bin day03`.

Day 5 doesn't work at the moment, so to run unit tests across the workspace,
use `cargo t --workspace --exclude day05`.
