# Prime Number Calculator - Linear Sieve Implementation

A high-performance Rust application that calculates all prime numbers up to `u64::MAX` using an optimized linear sieve algorithm with segmented processing and bit compression.

## Features

- **Linear Sieve Algorithm**: Uses the Euler sieve method (O(n) time complexity) for maximum efficiency
- **Segmented Processing**: Handles the massive `u64::MAX` range (18,446,744,073,709,551,615) without memory overflow
- **Bit Compression**: Optimized memory usage using compact bit array representation
- **Odd Number Optimization**: Processes only odd numbers to improve performance
- **Progress Monitoring**: Real-time progress display during computation
- **Verification**: Built-in validation to ensure correctness of calculated primes
- **GUI Interface**: Modern graphical user interface with eframe/egui
- **Dual Interface**: Both command-line and graphical versions available

## Quick Start

### Prerequisites

- Rust 1.70 or higher
- At least 2GB of available RAM
- 64-bit system (supporting `u64` type)

### Installation

```bash
git clone <repository-url>
cd 6
cargo build --release
```

### Usage

#### Command Line Interface (CLI)
```bash
./target/release/prime-cli.exe
```

#### Graphical User Interface (GUI)
```bash
./target/release/prime-gui.exe
```
or
```bash
cargo run --bin prime-gui
```

#### Development Builds
```bash
cargo run --bin prime-cli    # Command line version
cargo run --bin prime-gui    # GUI version
```

The CLI program will:
1. Display configuration information
2. Calculate primes up to user-specified range
3. Show progress during computation
4. Display results and verification status

The GUI program provides:
1. Interactive input with real-time validation
2. Visual prime/composite distinction with color coding
3. Special testing for algorithm verification
4. Filtered range display options

## Algorithm Details

### Linear Sieve (Euler's Sieve)

Unlike the traditional Eratosthenes sieve, the linear sieve ensures each composite number is marked exactly once, achieving true linear time complexity:

```rust
// Core principle: each composite is marked by its smallest prime factor
for i in 2..n {
    if is_prime[i] {
        primes.push(i);
    }
    for &prime in primes.iter() {
        if i * prime > n {
            break;
        }
        is_prime[i * prime] = false;
        if i % prime == 0 {
            break;
        }
    }
}
```

### Segmented Processing

Since `u64::MAX` is extremely large, the algorithm processes the range in segments:

1. **Precompute Base Primes**: Calculate all primes up to `√max_value`
2. **Segment Sieving**: Use base primes to sieve each segment
3. **Bit Compression**: Store only odd numbers in compressed bit arrays
4. **Progressive Processing**: Handle segments sequentially to minimize memory usage

### Memory Optimization

- **Bit Compression**: 64 numbers stored per 64-bit word
- **Odd-Only Storage**: Excludes even numbers (except 2)
- **Segment Size Tuning**: Optimal 500,000 odd numbers per segment

## Performance

### Current Implementation

- **Range**: 1,000,000,000 (test range)
- **Algorithm**: Segmented linear sieve with bit compression
- **Memory Usage**: ~8MB per segment
- **Expected Performance**: ~10M numbers/second on modern hardware

### Scaling to u64::MAX

- **Estimated Time**: Several days to weeks for complete `u64::MAX` range
- **Peak Memory**: ~100MB (segmented approach)
- **Theoretical Prime Count**: ~4.2 × 10¹⁷ primes

## Verification

The program includes comprehensive validation:

1. **First 10 Primes**: Verifies against known values `[2, 3, 5, 7, 11, 13, 17, 19, 23, 29]`
2. **Prime Number Theorem**: Compares actual count with mathematical approximation
3. **Composite Detection**: Ensures known composites are not marked as primes
4. **Prime Validation**: Double-checks large primes using trial division

## Configuration

The main parameters are defined in `src/main.rs`:

```rust
const MAX_VALUE: u64 = 1_000_000_000;  // Calculation range
const ODDS_PER_SEGMENT: u64 = 500_000; // Segmentation parameter
```

## Dependencies

```toml
[dependencies]
rand = "0.8"
rayon = "1.5"
eframe = "0.27.2"
egui = "0.27.2"
egui_extras = "0.27.2"
serde = { version = "1.0", features = ["derive"] }
```

- **rand**: For potential randomness in testing
- **rayon**: For potential parallel computation optimizations
- **eframe/egui**: Modern immediate mode GUI framework
- **serde**: Serialization support for data persistence

## Project Structure

```
6/
├── src/
│   ├── main.rs              # CLI application logic
│   ├── lib.rs               # Library interface for tests
│   ├── gui_main.rs          # GUI application
│   └── prime_sieve.rs       # Core sieve algorithm
├── tests/
│   └── prime_sieve_tests.rs # Comprehensive test suite
├── target/                  # Build output directory
├── Cargo.toml              # Project dependencies
├── Cargo.lock              # Dependency versions
├── .gitignore              # Git ignore rules
├── README.md               # This file (English)
├── GUI_GUIDE.md            # GUI user guide
├── TEST_RESULTS.md         # Test results documentation
├── README_zh.md            # Chinese version
└── agent.md                # Additional documentation
```

## Mathematical Background

### Prime Number Theorem

The number of primes ≤ x is approximately:
```
π(x) ≈ x / ln(x)
```

For improved accuracy with larger x:
```
π(x) ≈ x / (ln(x) - 1)
```

### Complexity Analysis

- **Time Complexity**: O(n) for linear sieve
- **Space Complexity**: O(√n) + O(segment_size)
- **Memory per Segment**: O(segment_size / 64) bytes

## Troubleshooting

### Common Issues

1. **Build Failures**: Ensure Rust version ≥ 1.70
2. **Memory Issues**: Reduce `ODDS_PER_SEGMENT` if encountering out-of-memory errors
3. **Performance Issues**: Check system load and available RAM
4. **Verification Failures**: Algorithm may need adjustment for edge cases

### Debug Mode

For detailed debugging, use:
```bash
RUST_LOG=debug ./target/release/prime.exe
```

## Contributing

1. Fork the repository
2. Create a feature branch
3. Implement optimizations or fixes
4. Add comprehensive tests
5. Submit a pull request

## License

This project is licensed under the Apache License 2.0 - see the LICENSE file for details.

## Acknowledgments

- Euler's linear sieve algorithm
- Modern prime number computation techniques
- Rust's performance optimization capabilities

## 🖥️ GUI Features

The graphical interface provides enhanced user experience:

- **Real-time Calculation**: Visual feedback during computation
- **Color-coded Display**: Green for primes, gray for composites
- **Interactive Filtering**: Range-based result filtering
- **Built-in Testing**: Automatic verification of known problematic numbers
- **Progress Monitoring**: Visual progress indicators

For detailed GUI usage instructions, see [GUI_GUIDE.md](GUI_GUIDE.md).

## 🧪 Testing

Comprehensive test suite covering:
- Basic sieve functionality
- Edge cases and error handling
- Algorithm correctness verification
- Performance benchmarks
- Specific bug regression tests

Run tests with:
```bash
cargo test
```

See [TEST_RESULTS.md](TEST_RESULTS.md) for detailed test results.

---

**Note**: Computing all primes up to `u64::MAX` is an extremely resource-intensive task intended primarily for algorithm demonstration and benchmarking purposes. For most practical use, ranges up to 1 billion are recommended.