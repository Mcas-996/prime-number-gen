use std::time::Instant;
mod prime_sieve;

fn main() {
    println!("High Performance Prime Calculator - Fixed Version");
    println!("===========================================");

    // 获取用户输入的最大值
    println!("Enter max value (default: 100000000): ");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).ok();

    let max_value: u64 = input.trim().parse().unwrap_or(1_000_000_000).max(2);

    let start_time = Instant::now();

    println!(
        "Starting calculation of all primes between 1 and {}",
        max_value
    );
    println!("Algorithm: Segmented Sieve + Bit Compression + Odd Optimization");
    println!();

    let primes = prime_sieve::segmented_sieve(max_value, 1_000_000);

    let duration = start_time.elapsed();

    println!();
    println!("Calculation completed!");
    println!("Total prime count: {}", primes.len());
    println!("Time taken: {:.2} seconds", duration.as_secs_f64());
    println!(
        "Processing speed: {:.0} numbers/second",
        max_value as f64 / duration.as_secs_f64()
    );

    // 验证素数的正确性
    println!("\nVerification results:");

    // 检查前10个素数是否正确
    let expected_first_10 = vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29];
    let first_10_correct = primes.iter().take(10).eq(&expected_first_10);

    if first_10_correct {
        println!("  PASS First 10 primes are correct");
    } else {
        println!("  FAIL First 10 primes are incorrect");
        println!("  Expected: {:?}", expected_first_10);
        println!("  Actual: {:?}", &primes[..10.min(primes.len())]);
    }

    // 检查最后几个素数
    if primes.len() >= 5 {
        println!("\nLast 5 primes:");
        for (i, &prime) in primes.iter().rev().take(5).enumerate() {
            println!("  Prime #{}: {}", primes.len() - i, prime);

            // 验证是否真的是素数
            if prime > 1 {
                let mut is_prime = true;
                let sqrt_p = (prime as f64).sqrt() as u64 + 1;
                for test_prime in prime_sieve::precompute_small_primes(sqrt_p) {
                    if test_prime * test_prime > prime {
                        break;
                    }
                    if prime % test_prime == 0 && test_prime != prime {
                        is_prime = false;
                        break;
                    }
                }
                if is_prime {
                    println!("          PASS {} is prime", prime);
                } else {
                    println!("          FAIL {} is not prime!", prime);
                }
            }
        }
    }

    // 与理论估计对比
    let estimated_count = prime_sieve::estimate_pi(max_value as f64);
    let error_percent =
        ((primes.len() as f64 - estimated_count as f64) / estimated_count as f64 * 100.0).abs();

    println!("\nTheoretical estimate: {} primes", estimated_count);
    println!("Actual calculation: {} primes", primes.len());
    println!("Error: {:.2}%", error_percent);

    // 额外验证：检查一些合数是否错误地被当做素数
    println!("\n额外验证 - 检查已知的合数:");
    let test_composites = vec![4, 6, 8, 9, 10, 12, 15, 21, 25, 27, 33, 35, 39, 49];
    for &composite in &test_composites {
        if composite <= max_value && primes.contains(&composite) {
            println!("  FAIL Incorrectly marked {} as prime", composite);
        }
    }

    // 检查一些已知的素数
    println!("额外验证 - 检查已知的素数:");
    let test_primes = vec![
        11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71,
    ];
    for &prime in &test_primes {
        if prime <= max_value && !primes.contains(&prime) {
            println!("  FAIL Incorrectly marked {} as composite", prime);
        }
    }

    if first_10_correct && error_percent < 5.0 {
        println!("\n✅ Algorithm verification passed!");
    } else {
        println!("\n❌ Algorithm needs further fixes!");
    }
}
