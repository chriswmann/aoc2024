# Day 3 requires nightly
run-day day:
  cargo {{ if day == "03" { "+nightly r --bin day" } else { "r --bin day" } }}{{day}}


run-tests:
  cargo +nightly t --workspace

add-day day:
  cargo new day{{day}}
  echo "santas_little_helpers = { workspace = true }" >> day{{day}}/Cargo.toml
  cp main_template.rs day{{day}}/src/main.rs

