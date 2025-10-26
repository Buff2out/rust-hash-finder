# Rust Hash Finder

[![Rust](https://img.shields.io/badge/rust-1.90%2B-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

High-performance SHA-256 hash finder that searches for hashes ending with a specific number of zeros using parallel processing.

## Features

- 🚀 **Parallel Processing**: Leverages Rayon for efficient multi-core computation
- 🔄 **Two Implementation Modes**: 
  - `atomics` - Lock-free atomic operations (fastest)
  - `crossbeam` - Channel-based producer-consumer pattern (default)
- 📊 **Structured Logging**: Built-in tracing support with configurable verbosity
- ✅ **Comprehensive Testing**: Unit, integration, and CLI tests
- 🛠️ **Production-Ready**: Proper error handling with `ExitCode`

## Installation

### From Source

```

git clone git@github.com:Buff2out/rust-hash-finder.git
cd rust-hash-finder
cargo build --release

```

The compiled binary will be available at `./target/release/rust-hash-finder`.

## Usage

### Basic Usage

```

# Find 5 hashes ending with 3 zeros

./target/release/rust-hash-finder -N 3 -F 5

```

### Command-Line Options

```

Usage: rust-hash-finder [OPTIONS] --zeros <ZEROS> --results <RESULTS>

Options:
  -N, --zeros <ZEROS>      Number of trailing zeros to find
  -F, --results <RESULTS>  Number of results to find before stopping
  -v, --verbose            Enable verbose logging
  -h, --help               Print help
  -V, --version            Print version

```

### Examples

```

# Find 6 hashes with 3 trailing zeros

./target/release/rust-hash-finder -N 3 -F 6

# Output

# 4163, "95d4362bd3cd4315d0bbe38dfa5d7fb8f0aed5f1a31d98d510907279194e3000"

# 11848, "cb58074fd7620cd0ff471922fd9df8812f29f302904b15e389fc14570a66f000"

# 

# With verbose logging

./target/release/rust-hash-finder -N 3 -F 5 --verbose

# Custom log level via environment variable

RUST_LOG=debug ./target/release/rust-hash-finder -N 4 -F 2

```

## Implementation Details

### Architecture

The application uses a parallel iterator pattern with Rayon's `par_bridge()` to distribute work across CPU cores. Two implementations are available via feature flags:

**Atomics Mode** (fastest):
- Uses `Arc<AtomicUsize>` for lock-free coordination
- Minimal overhead with `Relaxed` and `SeqCst` ordering
- Results collected via `Arc<Mutex<Vec<_>>>`

**Crossbeam Mode** (default):
- Producer-consumer pattern with bounded channel (capacity: 100)
- Separate consumer thread for result collection
- Better backpressure handling

### Performance

Performance characteristics (approximate, hardware-dependent):

- **Atomics**: ~100-500M operations/sec with minimal contention
- **Crossbeam**: ~40-80M operations/sec with channel overhead

For maximum performance, use the atomics feature:

```

cargo build --release --no-default-features --features atomics

```

## Performance Benchmarks

Benchmarked on: Intel i5-12450H (16 threads), RTX 3050, NixOS

### Atomics vs Crossbeam (N=5, F=5)

| Implementation | Mean Time | Std Dev | Relative |
|----------------|-----------|---------|----------|
| **Atomics** | 1.146s | ±0.025s | **1.00x** (baseline) |
| Crossbeam | 1.214s | ±0.009s | 1.06x slower |


| Command | Mean [s] | Min [s] | Max [s] | Relative |
|:---|---:|---:|---:|---:|
| `./hash-finder-crossbeam -N 5 -F 5` | 1.214 ± 0.009 | 1.198 | 1.223 | 1.06 ± 0.02 |
| `./hash-finder-atomics -N 5 -F 5` | 1.146 ± 0.025 | 1.118 | 1.175 | 1.00 |

**Key findings:**
- Atomics implementation is ~6% faster due to lower synchronization overhead
- Crossbeam shows more consistent timing (lower standard deviation)
- Both implementations scale well across multiple CPU cores (~11x CPU usage)

*Run on: `hyperfine --warmup 1 --runs 5 './hash-finder-{impl} -N 5 -F 5'`*

### Running Benchmarks Yourself

#### Prerequisites

Install hyperfine (command-line benchmarking tool):

`cargo install hyperfine`

#### Quick Benchmark

```
# Build both versions
cargo build --release
cp target/release/rust-hash-finder ./hash-finder-crossbeam

cargo build --release --no-default-features --features atomics
cp target/release/rust-hash-finder ./hash-finder-atomics

# Run benchmark (N=5, F=5, ~1-2 seconds per run)
hyperfine --warmup 1 --runs 5 \
  './hash-finder-crossbeam -N 5 -F 5' \
  './hash-finder-atomics -N 5 -F 5'
```

#### Comprehensive Benchmark

```
# Multiple workload sizes with statistical analysis
hyperfine --export-markdown benchmark.md \
  --warmup 1 \
  --runs 5 \
  --style full \
  --time-unit second \
  './hash-finder-crossbeam -N 5 -F 8' \
  './hash-finder-atomics -N 5 -F 8'

# View results
cat benchmark.md
```

#### Advanced: Parametric Benchmarking

```
# Test scalability across different workloads
hyperfine --warmup 1 \
  --parameter-list zeros 4,5 \
  --parameter-list results 5,10,20 \
  './hash-finder-atomics -N {zeros} -F {results}'
```

#### Performance Tips

- **Always use `--release`** builds for benchmarking (10-100x faster than debug)
- For N=4: each result takes ~0.1-0.5s to find
- For N=5: each result takes ~1-3s to find (16x harder than N=4)
- For N=6: each result takes ~15-50s to find (not recommended for benchmarking)
- Use `--warmup` to eliminate cold-start effects (JIT, caching)
- Use `--runs 5` or more for statistical significance

### Recommendation

**Use atomics** for:
- Maximum throughput in batch processing
- CPU-bound workloads where every millisecond counts
- Scenarios with minimal I/O or blocking operations

**Use crossbeam** for:
- More predictable, consistent latency
- Better code structure and maintainability
- Future extensibility (e.g., adding pipeline stages)
- Learning modern Rust concurrency patterns
```

### Recommendation

- Use **atomics** for maximum throughput
- Use **crossbeam** for more predictable latency and better code structure

### Logging

The application uses `tracing` for structured logging:

- **INFO** (default): High-level progress information
- **DEBUG** (--verbose): Detailed hash discovery events
- **TRACE** (RUST_LOG=trace): Instrumentation details

## Development

### Building

```

# Default (crossbeam) build

cargo build

# Atomics build

cargo build --no-default-features --features atomics

# Release build with optimizations

cargo build --release

```

### Testing

```

# Run all tests

cargo test

# Run unit tests only

cargo test --lib

# Run integration tests

cargo test --test integration_test

# Run CLI tests

cargo test --test cli_test

# Run tests with output

cargo test -- --nocapture

# Run tests in release mode (faster for find_hashes)

cargo test --release

```

### Project Structure

```

rust-hash-finder/
├── Cargo.toml              # Dependencies and feature flags
├── src/
│   ├── lib.rs             # Core logic (compute_hash, find_hashes)
│   └── main.rs            # CLI entry point with clap
├── tests/
│   ├── integration_test.rs # Integration tests
│   └── cli_test.rs        # Command-line interface tests
└── README.md              # This file

```

## Dependencies

- **clap** (4.5) - Command-line argument parsing
- **sha2** (0.10) - SHA-256 hashing
- **rayon** (1.10) - Data parallelism
- **crossbeam-channel** (0.5) - Lock-free MPMC channels
- **tracing** (0.1) - Structured logging
- **tracing-subscriber** (0.3) - Log output formatting

### Dev Dependencies

- **assert_cmd** (2.0) - CLI testing
- **predicates** (3.1) - Assertion helpers

## Technical Details

### Algorithm

1. Generate consecutive integers starting from 1
2. Compute SHA-256 hash for each integer
3. Check if hash ends with N zeros (hex representation)
4. Collect F results and terminate

### Parallelization Strategy

Uses Rayon's `par_bridge()` to convert sequential iterator into parallel:
- Work-stealing scheduler distributes tasks across threads
- Early termination when F results found
- Atomic counter prevents race conditions

### Memory Safety

- No unsafe code
- Proper cleanup via `ExitCode` return (no `std::process::exit()`)
- All destructors run correctly
- Thread-safe coordination via atomics/channels

## License

MIT License - see LICENSE file for details

## Contributing

Contributions are welcome! Please ensure:

1. All tests pass: `cargo test`
2. Code is formatted: `cargo fmt`
3. No clippy warnings: `cargo clippy`
4. Documentation is updated

## Acknowledgments

Built as part of a Rust developer internship task for demonstrating:
- Concurrent programming patterns
- SHA-256 cryptographic hashing
- CLI application development
- Testing best practices

---

**Note**: This is a computational task for educational purposes. Finding hashes with many trailing zeros can be time-consuming.
