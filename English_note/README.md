# Linear Sieve Algorithm for All Primes Within u64 Range

This program uses the linear sieve algorithm (also known as Euler's sieve) to calculate all prime numbers within the u64 range.

## Features

- Efficient linear sieve algorithm
- Supports calculating all primes within u64::MAX range
- Segmented processing of large number ranges to avoid memory overflow
- Progress display every 2 seconds for monitoring long-running operations
- Displays first 10 and last 10 primes as examples

## Build and Run

### Build

```bash
cargo build --release
```

### Run

```bash
./target/release/output_text.exe
```

## Algorithm Description

### Linear Sieve Algorithm

The linear sieve is an efficient prime sieving algorithm with time complexity O(n) and space complexity O(n). Unlike the Eratosthenes sieve, the linear sieve ensures each composite number is marked only once, making it more efficient.

### Segmented Processing

Since u64::MAX is a very large value (18,446,744,073,709,551,615), it's impossible to directly create such a large array. Therefore, the program adopts segmented processing:

1. First calculate all primes up to sqrt(u64::MAX)
2. Then use these small primes to screen subsequent intervals

### Progress Display

The program displays the current interval being processed and the number of primes found every 2 seconds, making it easy to monitor the progress of long-running operations.

## System Requirements

- Rust 1.70 or higher
- At least 2GB of available memory (actual usage depends on processing progress)
- System supporting u64 type (most modern systems support this)

## Performance Notes

Calculating all primes within the u64 range is an extremely time-consuming task that may take days or even weeks to complete. This program is mainly intended for algorithm demonstration and testing.

## Important Notes

1. The program runs for a very long time and is not recommended for production environments
2. Due to the enormous number of primes in the u64 range, the final results will consume significant memory
3. The program can be interrupted at any time, and calculated primes will be lost

## License

This project is licensed under the Apache License 2.0 - see the [LICENSE](../LICENSE) file for details.