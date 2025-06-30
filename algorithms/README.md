# Algorithms in Rust

This project is my journey into learning Rust programming language through implementing various algorithms and data structures. 

## Project Structure

```
algorithms/
├── Cargo.toml              # Project configuration and dependencies
├── README.md               # This file
├── src/
    ├── lib.rs              # Library root, exports all modules
    ├── <algorithm>.rs      # Various algorithms implementation
    └── bin/
        └── <algorithm>.rs  # Binary to execute algorithms
```

## How to Run

### Running Tests

To run all tests for the algorithms:

```bash
# Run all tests
cargo test --lib

# Run specific test
cargo test twosum
```

### Running Individual Algorithm Binaries

Each algorithm has a corresponding binary that demonstrates its usage:

```bash
# Run Two Sum binary
cargo run --bin twosum

# Run with release optimizations
cargo run --release --bin twosum
```
