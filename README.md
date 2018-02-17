[![MIT Licensed](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/jeremy-miller/exercism-rust/blob/master/LICENSE)
[![Rust Version](https://img.shields.io/badge/Rust-1.24.0-blue.svg)]()

# Exercism Rust
My solutions to Rust [Exercism](http://exercism.io/languages/rust/exercises) exercises.

## Table of Contents
- [Prerequisites](#prerequisites)
- [Documentation](#documentation)
- [Build](#build)
- [Dependencies](#dependencies)
- [Code Formatting](#code-formatting)
- [Static Code Analysis](#static-code-analysis)
- [Test](#test)
- [License](#license)

## Prerequisites
This tool requires at least Rust version 1.24.0 to be installed.

## Documentation
To build and view the documentation for the exercises in your browser, execute the following command:
```cargo doc --open```

## Build
To build the exercises, execute the following command:
```cargo build --all```

## Dependencies
To check for outdated dependencies, execute the following command:
```cargo outdated```

## Code Formatting
To run `rustfmt` on the exercises, execute the following steps:

1. Install `rustfmt`: ```rustup component add rustfmt-preview```
2. Run `rustfmt` on the exercises: ```cargo fmt --all```

## Static Code Analysis
To run `clippy` on the exercises, execute the following steps:

1. Install `clippy` (requires Rust [nightly](https://github.com/rust-lang-nursery/rustup.rs#working-with-nightly-rust)): ```cargo +nightly install clippy```
2. Run `clippy` on the `minigrep` codebase: ```cargo +nightly clippy --all```

## Test
To run all the tests, execute the following command:
```cargo test --all```

## License
[MIT](https://github.com/jeremy-miller/exercism-rust/blob/master/LICENSE)
