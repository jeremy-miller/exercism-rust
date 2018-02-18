# Gigasecond
Calculate the moment when someone has lived for 10<sup>9</sup> seconds.

[Source](http://exercism.io/exercises/rust/gigasecond/readme)

## Table of Contents
- [Documentation](#documentation)
- [Build](#build)
- [Dependencies](#dependencies)
- [Code Formatting](#code-formatting)
- [Static Code Analysis](#static-code-analysis)
- [Test](#test)
    - [Run Tests](#run-tests)
    - [Code Coverage](#code-coverage)
    - [Testify](#testify)

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
2. Run `rustfmt`: ```cargo fmt --all```

## Static Code Analysis
To run `clippy` on the exercises, execute the following steps:

1. Install `clippy` (requires Rust [nightly](https://github.com/rust-lang-nursery/rustup.rs#working-with-nightly-rust)): ```cargo +nightly install clippy```
2. Run `clippy`: ```cargo +nightly clippy --all```

## Test

### Run Tests
To run all the tests, execute the following command:
```cargo test --all```

### Code Coverage
To see code coverage of tests, execute the following steps:

1. On Ubuntu/Debian, install prerequisites: ```sudo apt update && sudo apt install libssl-dev pkg-config cmake zlib1g-dev```
2. Run `tarpaulin`: ```cargo tarpaulin```

### Testify
During development, [`testify`](https://github.com/greyblake/cargo-testify) can be used to watch changes to source
code files and automatically run `cargo test`.  A desktop notification is used to notify the developer of test results.
To use `testify`, execute the following steps:

1. On Ubuntu/Debian, install prerequisites: ```sudo apt install -y libdbus-1-dev```
2. Run `testify`: ```cargo testify```
