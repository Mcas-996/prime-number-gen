use std::time::Instant;

// 位压缩筛法，更高效的内存使用
struct CompactBitSieve {
    data: Vec<u64>,
    size: usize,
}

impl CompactBitSieve {
    fn new(size: usize) -> Self {
        let vec_size = (size + 63) / 64;
        let data = vec![u64::MAX; vec_size]; // 全部初始化为1
        Self { data, size }
    }

    #[inline(always)]
    fn is_prime(&self, idx: usize) -> bool {
        if idx >= self.size {
            return false;
        }
        let word_idx = idx / 64;
        let bit_idx = idx % 64;
        (self.data[word_idx] >> bit_idx) & 1 == 1
    }

    #[inline(always)]
    fn set_composite(&mut self, idx: usize) {
        if idx >= self.size {
            return;
        }
        let word_idx = idx / 64;
        let bit_idx = idx % 64;
        self.data[word_idx] &= !(1u64 << bit_idx);
    }

    fn collect_primes(&self, start: u64) -> Vec<u64> {
        let mut primes = Vec::with_capacity(self.size / 8);

        for i in 0..self.size {
            if self.is_prime(i) {
                let num = start + (i as u64) * 2;
                if num > 2 {
                    primes.push(num);
                }
            }
        }
        primes
    }
}

// 高效的素数计数估计（基于素数定理）
fn estimate_pi(x: f64) -> u64 {
    if x < 2.0 {
        return 0;
    }
    // 使用改进的素数定理近似
    let x_ln = x.ln();
    if x >= 55.0 {
        (x / (x_ln - 1.0)) as u64
    } else {
        (x / x_ln) as u64
    }
}

// 预计算小素数，使用简单的筛法
fn precompute_small_primes(limit: u64) -> Vec<u64> {
    let limit = limit as usize;
    if limit < 2 {
        return vec![2];
    }

    let mut is_prime = vec![true; limit + 1];
    is_prime[0] = false;
    is_prime[1] = false;

    let mut primes = vec![2];
    for i in (3..=limit).step_by(2) {
        if is_prime[i] {
            primes.push(i as u64);
            if i * i <= limit {
                for j in (i * i..=limit).step_by(i * 2) {
                    is_prime[j] = false;
                }
            }
        }
    }

    primes
}

// 修复后的分段筛法实现
fn segmented_sieve(max_value: u64, _batch_size: u64) -> Vec<u64> {
    let start = Instant::now();

    // 预计算基准素数到sqrt(max_value)
    let sqrt_max = (max_value as f64).sqrt() as u64 + 1;
    println!("预计算素数到 sqrt({}) = {}...", max_value, sqrt_max);
    let base_primes = precompute_small_primes(sqrt_max);
    println!(
        "找到 {} 个基准素数，用时 {:?} ms",
        base_primes.len(),
        start.elapsed().as_millis()
    );

    // 移除2，因为我们只处理奇数
    let odd_base_primes: Vec<u64> = base_primes.iter().skip(1).copied().collect();

    let mut all_primes = Vec::with_capacity(estimate_pi(max_value as f64) as usize);
    all_primes.push(2);

    // 分段处理范围 [3, max_value]
    const ODDS_PER_SEGMENT: u64 = 500_000; // 减小段大小，确保正确性

    let segment_count = (max_value - 3) / (2 * ODDS_PER_SEGMENT) + 1;
    let current_time = Instant::now();
    let mut total_primes_found = 0u64;

    println!("开始分段处理，共 {} 个段", segment_count);

    // 分段处理
    for segment_idx in 0..segment_count {
        let segment_start = 3 + segment_idx * 2 * ODDS_PER_SEGMENT;
        let segment_end = std::cmp::min(segment_start + 2 * ODDS_PER_SEGMENT - 2, max_value);

        if segment_start > segment_end {
            break;
        }

        // 创建位筛（只包含奇数）
        let segment_size = ((segment_end - segment_start) / 2 + 1) as usize;
        let mut sieve = CompactBitSieve::new(segment_size);

        // 使用基准素数进行筛除
        for &prime in &odd_base_primes {
            let prime_squared = prime * prime;

            // 如果prime的平方已经超过段结束，跳过
            if prime_squared > segment_end {
                continue;
            }

            // 找到该质数在当前段中的第一个奇数倍数
            let first_multiple = if prime_squared >= segment_start {
                // 确保是奇数
                if prime_squared % 2 == 0 {
                    prime_squared + prime
                } else {
                    prime_squared
                }
            } else {
                // 计算第一个大于等于segment_start的prime的倍数
                let remainder = segment_start % prime;
                let mut first = if remainder == 0 {
                    segment_start
                } else {
                    segment_start + (prime - remainder)
                };

                // 确保是奇数倍数
                if first % 2 == 0 {
                    first += prime;
                }

                first
            };

            // 标记所有奇数倍数
            let mut multiple = first_multiple;
            while multiple <= segment_end {
                // 计算在位筛中的索引
                let odd_index = ((multiple - segment_start) / 2) as usize;
                if odd_index < segment_size {
                    sieve.set_composite(odd_index);
                }
                multiple += prime * 2; // 只标记奇数倍数
            }
        }

        // 收集当前段的素数
        let segment_primes = sieve.collect_primes(segment_start);
        all_primes.extend_from_slice(&segment_primes);
        total_primes_found += segment_primes.len() as u64;

        // 报告进度
        if segment_idx % 10 == 0 {
            let progress_percent = (segment_idx as f64 / segment_count as f64) * 100.0;
            let elapsed = current_time.elapsed().as_secs_f64();
            let rate = if elapsed > 0.0 {
                (segment_idx as f64 * ODDS_PER_SEGMENT as f64 * 2.0) / elapsed
            } else {
                0.0
            };

            println!(
                "[{}] {:.1}% {}M {:.0}n/s 段 {} 基准素数 {} 已找到素数 {}",
                "=".repeat((progress_percent as usize / 10).min(10)),
                progress_percent,
                (segment_idx * ODDS_PER_SEGMENT * 2) / 1_000_000,
                rate,
                segment_idx,
                odd_base_primes.len(),
                total_primes_found
            );
        }
    }

    all_primes
}

fn main() {
    println!("高性能素数计算器 - 修复版本");
    println!("============================");

    // 测试更大的范围以确保修复的完整性
    const MAX_VALUE: u64 = 1_000_000_000; // 1亿，验证大范围的正确性
    let start_time = Instant::now();

    println!("开始计算 1 到 {} 之间的所有素数", MAX_VALUE);
    println!("算法: 分段筛法 + 位压缩 + 奇数优化");
    println!();

    let primes = segmented_sieve(MAX_VALUE, 1_000_000);

    let duration = start_time.elapsed();

    println!();
    println!("计算完成！");
    println!("总素数数量: {}", primes.len());
    println!("用时: {:.2} 秒", duration.as_secs_f64());
    println!(
        "处理速度: {:.0} 数字/秒",
        MAX_VALUE as f64 / duration.as_secs_f64()
    );

    // 验证素数的正确性
    println!("\n验证结果:");

    // 检查前10个素数是否正确
    let expected_first_10 = vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29];
    let first_10_correct = primes.iter().take(10).eq(&expected_first_10);

    if first_10_correct {
        println!("  ✓ 前10个素数正确");
    } else {
        println!("  ✗ 前10个素数有误");
        println!("  预期: {:?}", expected_first_10);
        println!("  实际: {:?}", &primes[..10.min(primes.len())]);
    }

    // 检查最后几个素数
    if primes.len() >= 5 {
        println!("\n最后5个素数:");
        for (i, &prime) in primes.iter().rev().take(5).enumerate() {
            println!("  素数 #{}: {}", primes.len() - i, prime);

            // 验证是否真的是素数
            if prime > 1 {
                let mut is_prime = true;
                let sqrt_p = (prime as f64).sqrt() as u64 + 1;
                for test_prime in precompute_small_primes(sqrt_p) {
                    if test_prime * test_prime > prime {
                        break;
                    }
                    if prime % test_prime == 0 && test_prime != prime {
                        is_prime = false;
                        break;
                    }
                }
                if is_prime {
                    println!("          ✓ {} 是素数", prime);
                } else {
                    println!("          ✗ {} 不是素数！", prime);
                }
            }
        }
    }

    // 与理论估计对比
    let estimated_count = estimate_pi(MAX_VALUE as f64);
    let error_percent =
        ((primes.len() as f64 - estimated_count as f64) / estimated_count as f64 * 100.0).abs();

    println!("\n理论估计: {} 个素数", estimated_count);
    println!("实际计算: {} 个素数", primes.len());
    println!("误差: {:.2}%", error_percent);

    // 额外验证：检查一些合数是否错误地被当做素数
    println!("\n额外验证 - 检查已知的合数:");
    let test_composites = vec![4, 6, 8, 9, 10, 12, 15, 21, 25, 27, 33, 35, 39, 49];
    for &composite in &test_composites {
        if composite <= MAX_VALUE && primes.contains(&composite) {
            println!("  ✗ 错误地将 {} 标记为素数", composite);
        }
    }

    // 检查一些已知的素数
    println!("额外验证 - 检查已知的素数:");
    let test_primes = vec![
        11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71,
    ];
    for &prime in &test_primes {
        if prime <= MAX_VALUE && !primes.contains(&prime) {
            println!("  ✗ 错误地将 {} 标记为合数", prime);
        }
    }

    if first_10_correct && error_percent < 5.0 {
        println!("\n✅ 算法验证通过！");
    } else {
        println!("\n❌ 算法需要进一步修复！");
    }
}
