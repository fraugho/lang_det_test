# Language Detection Test

This project provides language detection functionality with both Rust native and C binding implementations.

## Usage

### Rust Native Version

To run the Rust native language detection test:

```bash
cd rust_test
cargo run --release
```

### C Binding Version

To build and run the C binding version:

1. First, build the Rust library:
```bash
cargo build --release
```

2. Navigate to the C directory and build:
```bash
cd c
cmake . && make && ./m
```

## Requirements

- Rust (latest stable version recommended)
- CMake
- C compiler (GCC or Clang)

## Project Structure

- Root directory: Rust native implementation
- `c/` directory: C bindings and test code

## Performance Metrics

### Lingua
- **Duration:** 52,675μs
- **Accuracy:** 68,693/70,000 (98.13%)

### WhichLang
- **Duration:** 95μs  
- **Accuracy:** 48,555/70,000 (69.36%)
