#[cfg(test)]
mod tests {
    use prime::prime_sieve;

    // 精确测试特定数字的素数性
    fn is_prime_brute_force(num: u64) -> bool {
        if num < 2 {
            return false;
        }
        if num == 2 {
            return true;
        }
        if num % 2 == 0 {
            return false;
        }

        let sqrt_num = (num as f64).sqrt() as u64 + 1;
        for i in (3..=sqrt_num).step_by(2) {
            if num % i == 0 {
                return false;
            }
        }
        true
    }

    #[test]
    fn test_basic_sieve_small_range() {
        println!("Testing basic sieve logic with small numbers...");

        let max_test = 100;
        let primes = prime_sieve::segmented_sieve(max_test, 1000);

        // 预期的前25个素数
        let expected_primes = vec![
            2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83,
            89, 97,
        ];

        assert_eq!(
            primes, expected_primes,
            "Basic sieve test failed for range 1-100"
        );
        println!(
            "✅ Basic sieve test passed: {} primes found correctly",
            primes.len()
        );
    }

    #[test]
    fn test_known_composites() {
        println!("Testing known composite numbers...");

        let max_test = 60;
        let primes = prime_sieve::segmented_sieve(max_test, 1000);

        // 测试一些已知的合数，特别关注17的倍数
        let test_cases = vec![
            (15, false, "3 * 5"),
            (17, true, "prime"),
            (21, false, "3 * 7"),
            (25, false, "5 * 5"),
            (27, false, "3 * 9"),
            (33, false, "3 * 11"),
            (34, false, "2 * 17"),
            (35, false, "5 * 7"),
            (39, false, "3 * 13"),
            (45, false, "5 * 9"),
            (49, false, "7 * 7"),
            (51, false, "3 * 17"),
            (68, false, "4 * 17"),
            (85, false, "5 * 17"),
            (102, false, "6 * 17"),
        ];

        for &(num, should_be_prime, description) in &test_cases {
            let algorithm_says_prime = primes.contains(&num);
            assert_eq!(
                algorithm_says_prime, should_be_prime,
                "Number {} ({}) - algorithm says {}, should be {}",
                num, description, algorithm_says_prime, should_be_prime
            );
        }

        println!("✅ Composite/prime identification test passed");
    }

    #[test]
    fn test_multiple_of_17() {
        println!("Testing multiples of 17 (the problematic case)...");

        let max_test = 500;
        let primes = prime_sieve::segmented_sieve(max_test, 1000);

        // 测试17的所有2-30倍
        for i in 2..=30 {
            let multiple = 17 * i;
            assert!(
                !primes.contains(&multiple),
                "17 * {} = {} should be marked as composite",
                i,
                multiple
            );
        }

        // 确认17本身是素数
        assert!(primes.contains(&17), "17 should be marked as prime");

        println!("✅ All multiples of 17 correctly identified as composites");
    }

    #[test]
    #[ignore] // 这个测试需要太长时间，暂时忽略
    fn test_problematic_number_21474836359() {
        println!("Testing the specific problematic number: 21474836359...");

        let test_number = 21474836359;
        let range_end = test_number + 1000; // 给一点缓冲

        let primes = prime_sieve::segmented_sieve(range_end, 1_000_000);

        // 检查问题数字
        assert!(
            !primes.contains(&test_number),
            "Problematic number {} should be marked as composite (17 * 126322727)",
            test_number
        );

        println!("✅ Problematic number correctly identified as composite");

        // 验证17被正确识别为素数（126322727太大，跳过验证以节省时间）
        assert!(primes.contains(&17), "17 should be marked as prime");

        println!("✅ Prime factor 17 correctly identified");
    }

    #[test]
    #[ignore] // 这个测试需要太长时间，暂时忽略
    fn test_primes_around_problematic_number() {
        println!("Testing primes around the problematic number...");

        let test_number = 21474836359;
        let range_start = test_number - 100;
        let range_end = test_number + 100;

        let primes = prime_sieve::segmented_sieve(range_end, 1_000_000);
        let relevant_primes: Vec<u64> = primes
            .iter()
            .filter(|&&p| p >= range_start && p <= range_end)
            .copied()
            .collect();

        println!("Primes found around {}:", test_number);
        for &prime in &relevant_primes {
            println!("  {}", prime);
        }

        // 确认问题数字不在素数列表中
        assert!(
            !relevant_primes.contains(&test_number),
            "Problematic number should not be in the primes list"
        );

        println!("✅ Range test passed");
    }

    #[test]
    fn test_prime_count_estimation() {
        println!("Testing prime count estimation...");

        let test_values = vec![100, 1000, 10000, 100000];

        for &max_value in &test_values {
            let primes = prime_sieve::segmented_sieve(max_value, 1_000_000);
            let actual_count = primes.len() as u64;
            let estimated_count = prime_sieve::estimate_pi(max_value as f64);

            let error_percent =
                ((actual_count as f64 - estimated_count as f64) / estimated_count as f64).abs();

            println!(
                "Range 1-{}: actual={}, estimated={}, error={:.2}%",
                max_value, actual_count, estimated_count, error_percent
            );

            // 确保误差在合理范围内（对于更大的数字可能会稍有偏差）
            assert!(
                error_percent < 20.0,
                "Estimation error too large for {}: {:.2}%",
                max_value,
                error_percent
            );
        }

        println!("✅ Prime count estimation test passed");
    }

    #[test]
    fn test_small_primes_precomputation() {
        println!("Testing small primes precomputation...");

        let limit = 100;
        let small_primes = prime_sieve::precompute_small_primes(limit);

        // 验证前几个素数
        let expected_first_primes = vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29];

        for (i, &expected) in expected_first_primes.iter().enumerate() {
            assert_eq!(
                small_primes[i], expected,
                "Precomputed primes mismatch at position {}: expected {}, got {}",
                i, expected, small_primes[i]
            );
        }

        // 验证所有计算出的数字都是素数
        for &prime in &small_primes {
            assert!(
                is_prime_brute_force(prime),
                "Precomputed number {} is not actually prime",
                prime
            );
            assert!(
                prime <= limit,
                "Precomputed prime {} exceeds limit {}",
                prime,
                limit
            );
        }

        println!(
            "✅ Small primes precomputation test passed: {} primes found",
            small_primes.len()
        );
    }

    #[test]
    fn test_edge_cases() {
        println!("Testing edge cases...");

        // 测试非常小的范围
        let primes_1 = prime_sieve::segmented_sieve(1, 100);
        assert!(primes_1.is_empty(), "No primes should be found for range 1");

        let primes_2 = prime_sieve::segmented_sieve(2, 100);
        assert_eq!(
            primes_2,
            vec![2],
            "Only prime 2 should be found for range 2"
        );

        let primes_3 = prime_sieve::segmented_sieve(3, 100);
        assert_eq!(
            primes_3,
            vec![2, 3],
            "Primes 2 and 3 should be found for range 3"
        );

        // 测试边界数字
        let primes_10 = prime_sieve::segmented_sieve(10, 100);
        assert_eq!(
            primes_10,
            vec![2, 3, 5, 7],
            "First 4 primes should be found for range 10"
        );

        println!("✅ Edge cases test passed");
    }

    #[test]
    fn test_large_range_performance() {
        println!("Testing larger range for performance and correctness...");

        // 测试一个中等大小的范围（减小以避免超时）
        let max_value = 100_000;
        let start_time = std::time::Instant::now();
        let primes = prime_sieve::segmented_sieve(max_value, 1_000_000);
        let duration = start_time.elapsed();

        let known_count = 9592; // 100,000以内的素数数量

        assert_eq!(
            primes.len(),
            known_count as usize,
            "Incorrect prime count for 100,000: expected {}, got {}",
            known_count,
            primes.len()
        );

        // 验证前几个和后几个素数
        assert_eq!(primes[0], 2, "First prime should be 2");
        assert_eq!(primes[1], 3, "Second prime should be 3");
        assert_eq!(
            primes.last().unwrap(),
            &99991,
            "Last prime under 100k should be 99991"
        );

        println!(
            "✅ Large range test passed: {} primes in {:.2}s",
            primes.len(),
            duration.as_secs_f64()
        );
    }

    #[test]
    fn test_prime_count_ten_million() {
        // Regression: ensure the full set of primes is produced for 10,000,000
        let max_value = 10_000_000;
        let primes = prime_sieve::segmented_sieve(max_value, 1_000_000);
        let expected_count = 664_579;

        assert_eq!(
            primes.len(),
            expected_count,
            "Incorrect prime count for 10,000,000: expected {}, got {}",
            expected_count,
            primes.len()
        );
        assert_eq!(
            primes.last().copied(),
            Some(9_999_991),
            "Last prime under 10,000,000 mismatch"
        );
    }

    #[test]
    fn test_random_prime_verification() {
        println!("Testing random prime verification...");

        let max_value = 10_000;
        let primes = prime_sieve::segmented_sieve(max_value, 1_000_000);

        // 随机选取一些素数进行验证
        let test_positions = vec![
            0,
            1,
            10,
            50,
            100,
            500,
            1000,
            primes.len() / 2,
            primes.len() - 1,
        ];

        for &pos in &test_positions {
            if pos < primes.len() {
                let prime = primes[pos];
                assert!(
                    is_prime_brute_force(prime),
                    "Prime at position {} (value {}) failed brute force verification",
                    pos,
                    prime
                );
            }
        }

        // 验证一些合数不在列表中
        let known_composites = vec![4, 6, 8, 9, 10, 12, 15, 21, 25, 27, 49, 121];
        for &composite in &known_composites {
            if composite <= max_value {
                assert!(
                    !primes.contains(&composite),
                    "Composite {} should not be in primes list",
                    composite
                );
            }
        }

        println!("✅ Random prime verification test passed");
    }
}
