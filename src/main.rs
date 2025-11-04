use std::time::Instant;
use std::io::{self, Write};

fn main(){
    println!("Calculating all prime numbers up to u64::MAX using linear sieve...");
    let start_time = Instant::now();
    
    // 使用线性筛法计算所有u64以内的素数
    let primes = linear_sieve_u64();
    
    let duration = start_time.elapsed();
    
    println!("\nGenerated {} prime numbers in {:.2} seconds:", primes.len(), duration.as_secs_f64());
    println!("First 10 primes:");
    for (i, prime) in primes.iter().take(10).enumerate() {
        println!("Prime {}: {}", i + 1, prime);
    }
    println!("Last 10 primes:");
    for (i, prime) in primes.iter().rev().take(10).enumerate() {
        println!("Prime #{}: {}", primes.len() - i, prime);
    }
}

// 使用线性筛法计算所有u64以内的素数
fn linear_sieve_u64() -> Vec<u64> {
    // u64的最大值
    let max_value = u64::MAX;
    
    // 由于u64::MAX太大，无法直接创建这么大的数组
    // 我们需要分段处理
    
    // 首先计算sqrt(u64::MAX)以内的素数
    let sqrt_max = (max_value as f64).sqrt() as u64;
    println!("Calculating primes up to sqrt(u64::MAX) = {}...", sqrt_max);
    let small_primes = linear_sieve_segment(0, sqrt_max);
    println!("Found {} small primes.", small_primes.len());
    
    // 使用分段筛法计算剩余区间的素数
    let segment_size = 1_000_000; // 每段100万个数字
    let mut all_primes = small_primes.clone();
    let mut current = sqrt_max + 1;
    
    // 记录上次显示进度的时间
    let mut last_progress_time = std::time::Instant::now();
    let progress_interval = std::time::Duration::from_secs(2); // 2秒间隔
    let mut showed_progress = false;
    
    while current <= max_value {
        let end = std::cmp::min(current + segment_size - 1, max_value);
        
        // 检查是否需要显示进度
        let now = std::time::Instant::now();
        if now.duration_since(last_progress_time) >= progress_interval {
            print!("Processing segment {} to {}...", current, end);
            io::stdout().flush().unwrap();
            last_progress_time = now;
            showed_progress = true;
        }
        
        let segment_primes = linear_sieve_segment_with_small_primes(current, end, &small_primes);
        all_primes.extend_from_slice(&segment_primes);
        
        // 如果刚才显示了进度，现在显示结果
        if showed_progress {
            println!(" Found {} primes. Total: {}", segment_primes.len(), all_primes.len());
            showed_progress = false;
        }
        
        // 防止溢出
        if end == max_value {
            break;
        }
        current = end + 1;
    }
    
    all_primes
}

// 线性筛法实现，用于计算指定范围内的素数
fn linear_sieve_segment(start: u64, end: u64) -> Vec<u64> {
    let mut primes = Vec::new();
    let size = (end - start + 1) as usize;
    let mut is_prime = vec![true; size];
    
    // 特殊处理0和1
    if start == 0 && size > 1 {
        is_prime[0] = false; // 0
        is_prime[1] = false; // 1
    } else if start == 1 {
        is_prime[0] = false; // 1
    }
    
    for i in 2..=end {
        let i_index = if i >= start { (i - start) as usize } else { continue };
        
        if is_prime[i_index] {
            primes.push(i);
            
            // 标记i的倍数为非素数
            let mut multiple = i * i;
            while multiple <= end {
                if multiple >= start {
                    let multiple_index = (multiple - start) as usize;
                    is_prime[multiple_index] = false;
                }
                // 防止溢出
                if multiple > u64::MAX / i {
                    break;
                }
                multiple += i;
            }
        }
    }
    
    primes
}

// 使用已知的小素数来筛分段
fn linear_sieve_segment_with_small_primes(start: u64, end: u64, small_primes: &[u64]) -> Vec<u64> {
    let mut primes = Vec::new();
    let size = (end - start + 1) as usize;
    let mut is_prime = vec![true; size];
    
    // 特殊处理0和1
    if start == 0 && size > 1 {
        is_prime[0] = false; // 0
        is_prime[1] = false; // 1
    } else if start == 1 {
        is_prime[0] = false; // 1
    }
    
    // 使用已知的小素数来筛
    for &p in small_primes {
        if p * p > end {
            break;
        }
        
        // 计算第一个大于等于start的p的倍数
        let mut multiple = if start % p == 0 {
            start
        } else {
            start + (p - start % p)
        };
        
        // 确保multiple至少是p*p
        if multiple < p * p {
            multiple = p * p;
        }
        
        // 标记所有p的倍数为非素数
        while multiple <= end {
            let multiple_index = (multiple - start) as usize;
            is_prime[multiple_index] = false;
            
            // 防止溢出
            if multiple > u64::MAX - p {
                break;
            }
            multiple += p;
        }
    }
    
    // 收集素数
    for i in start..=end {
        let i_index = (i - start) as usize;
        if is_prime[i_index] {
            primes.push(i);
        }
    }
    
    primes
}