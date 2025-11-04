use std::time::Instant;
use std::io::{self, Write};

fn main(){
    println!("Calculating all prime numbers up to u64::MAX using optimized linear sieve...");
    let start_time = Instant::now();
    
    // 使用优化的线性筛法计算所有u64以内的素数
    let primes = optimized_linear_sieve_u64();
    
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

// 使用优化的线性筛法计算所有u64以内的素数
fn optimized_linear_sieve_u64() -> Vec<u64> {
    // u64的最大值
    let max_value = u64::MAX;
    
    // 由于u64::MAX太大，无法直接创建这么大的数组
    // 我们需要分段处理
    
    // 首先计算sqrt(u64::MAX)以内的素数
    let sqrt_max = (max_value as f64).sqrt() as u64;
    println!("Calculating primes up to sqrt(u64::MAX) = {}...", sqrt_max);
    let small_primes = optimized_linear_sieve_segment(0, sqrt_max);
    println!("Found {} small primes.", small_primes.len());
    
    // 使用分段筛法计算剩余区间的素数
    // 使用动态分段大小，根据系统可用内存调整
    let segment_size = calculate_optimal_segment_size();
    println!("Using segment size: {}", segment_size);
    
    let mut all_primes = small_primes.clone();
    let mut current = sqrt_max + 1;
    
    // 记录上次显示进度的时间
    let mut last_progress_time = std::time::Instant::now();
    let progress_interval = std::time::Duration::from_secs(2); // 2秒间隔
    let mut showed_progress = false;
    
    // 用于统计两秒内处理的数字数量
    let mut numbers_processed = 0u64;
    let mut last_numbers_processed = 0u64;
    
    // 添加内存使用监控
    let mut segments_processed = 0u64;
    const MAX_SEGMENTS_BEFORE_GC: u64 = 10; // 处理10个分段后强制垃圾回收
    
    // 添加性能统计
    let mut total_primes_found = small_primes.len() as u64;
    let mut processing_start_time = std::time::Instant::now();
    
    // 预先计算所有需要用到的小素数的平方，避免重复计算
    let small_primes_squares: Vec<u64> = small_primes.iter()
        .filter(|&&p| p > 2) // 跳过2
        .map(|&p| p * p)
        .collect();
    
    while current <= max_value {
        let end = std::cmp::min(current + segment_size - 1, max_value);
        
        // 统计处理的数字数量
        numbers_processed += end - current + 1;
        
        // 检查是否需要显示进度
        let now = std::time::Instant::now();
        if now.duration_since(last_progress_time) >= progress_interval {
            // 计算处理速度和进度百分比
            let elapsed = now.duration_since(processing_start_time).as_secs_f64();
            let speed = if elapsed > 0.0 { numbers_processed as f64 / elapsed } else { 0.0 };
            let progress_percent = if max_value > 0 { 
                (end as f64 / max_value as f64) * 100.0 
            } else { 
                0.0 
            };
            
            // 清除上一行进度
            print!("\r\x1b[2K"); // \r 回到行首，\x1b[2K 清除整行
            // 显示当前进度和两秒内处理的数字数量
            print!("Progress: {:.4}% | Segment {} to {} | Checked {} numbers in last 2s | Speed: {:.0} nums/s | Total primes: {}", 
                   progress_percent, current, end, numbers_processed - last_numbers_processed, speed, total_primes_found);
            io::stdout().flush().unwrap();
            last_progress_time = now;
            showed_progress = true;
            last_numbers_processed = numbers_processed;
        }
        
        let segment_primes = optimized_linear_sieve_segment_with_small_primes(
            current, end, &small_primes, &small_primes_squares
        );
        total_primes_found += segment_primes.len() as u64;
        all_primes.extend_from_slice(&segment_primes);
        
        // 如果刚才显示了进度，现在显示结果
        if showed_progress {
            // 清除当前行并显示结果
            print!("\r");
            print!("Progress: {:.4}% | Segment {} to {} | Found {} primes. Total: {}", 
                    (end as f64 / max_value as f64) * 100.0, current, end, segment_primes.len(), all_primes.len());
            showed_progress = false;
        }
        
        // 增加分段计数器
        segments_processed += 1;
        
        // 定期触发垃圾回收以释放内存
        if segments_processed % MAX_SEGMENTS_BEFORE_GC == 0 {
            // 在Rust中，我们通过创建新的作用域来帮助释放内存
            println!("\nMemory checkpoint: processed {} segments, total primes: {}", 
                    segments_processed, all_primes.len());
        }
        
        // 防止溢出
        if end == max_value {
            break;
        }
        current = end + 1;
    }
    
    all_primes
}

// 计算最优分段大小，基于系统可用内存
fn calculate_optimal_segment_size() -> u64 {
    // 基本分段大小，可以根据系统内存调整
    let base_segment_size = 200_000_000; // 2亿个数字
    
    // 对于每个奇数，我们需要1个bool（1字节）
    // 所以2亿个数字需要约100MB内存（只处理奇数）
    
    // 在实际应用中，可以通过系统API获取可用内存
    // 这里我们使用一个保守的估计
    base_segment_size
}

// 优化的线性筛法实现，用于计算指定范围内的素数
// 使用真正的线性筛法（欧拉筛），并跳过2以外的偶数
fn optimized_linear_sieve_segment(start: u64, end: u64) -> Vec<u64> {
    let mut primes = Vec::with_capacity(estimate_prime_count(end - start + 1));
    
    // 特殊处理小范围
    if end < 2 {
        return primes;
    }
    
    // 调整start，确保至少从2开始
    let adjusted_start = std::cmp::max(start, 2);
    
    // 对于小范围，使用简单方法
    if end <= 100_000 {
        let size = (end - adjusted_start + 1) as usize;
        let mut is_prime = vec![true; size];
        
        // 特殊处理0和1（如果在范围内）
        if start <= 0 {
            let index = (0 - adjusted_start) as usize;
            is_prime[index] = false;
        }
        if start <= 1 {
            let index = (1 - adjusted_start) as usize;
            is_prime[index] = false;
        }
        
        // 线性筛法实现
        for i in adjusted_start..=end {
            let i_index = (i - adjusted_start) as usize;
            
            if is_prime[i_index] {
                primes.push(i);
                
                // 标记i的倍数为非素数，使用线性筛的方式
                for &p in primes.iter() {
                    let product = i * p;
                    if product > end {
                        break;
                    }
                    
                    let product_index = (product - adjusted_start) as usize;
                    is_prime[product_index] = false;
                    
                    // 确保每个合数只被标记一次
                    if i % p == 0 {
                        break;
                    }
                }
            }
        }
        
        return primes;
    }
    
    // 对于大范围，使用优化的方法
    // 首先处理2
    if adjusted_start <= 2 && end >= 2 {
        primes.push(2);
    }
    
    // 只处理奇数
    let odd_start = if adjusted_start <= 3 { 3 } else { adjusted_start | 1 }; // 确保是奇数
    if odd_start > end {
        return primes;
    }
    
    // 计算奇数数量
    let odd_count = ((end - odd_start) / 2 + 1) as usize;
    let mut is_prime_odd = vec![true; odd_count];
    
    // 线性筛法（只处理奇数）
    let mut odd_primes = Vec::new();
    
    for i in (odd_start..=end).step_by(2) {
        let i_index = ((i - odd_start) / 2) as usize;
        
        if is_prime_odd[i_index] {
            odd_primes.push(i);
            
            // 使用线性筛的方式标记倍数
            for &p in odd_primes.iter() {
                let product = i * p;
                if product > end {
                    break;
                }
                
                // 只标记奇数倍数
                if product % 2 == 1 {
                    let product_index = ((product - odd_start) / 2) as usize;
                    is_prime_odd[product_index] = false;
                }
                
                // 确保每个合数只被标记一次
                if i % p == 0 {
                    break;
                }
            }
        }
    }
    
    primes.extend_from_slice(&odd_primes);
    primes
}

// 使用已知的小素数来筛分段，优化版本
fn optimized_linear_sieve_segment_with_small_primes(
    start: u64, 
    end: u64, 
    small_primes: &[u64],
    small_primes_squares: &[u64]
) -> Vec<u64> {
    // 预估素数数量以减少内存重新分配
    let mut primes = Vec::with_capacity(estimate_prime_count(end - start + 1) / 10);
    
    // 特殊处理小范围
    if end < 2 {
        return primes;
    }
    
    // 调整start，确保至少从2开始
    let adjusted_start = std::cmp::max(start, 2);
    
    // 处理2
    if adjusted_start <= 2 && end >= 2 {
        primes.push(2);
    }
    
    // 只处理奇数
    let odd_start = if adjusted_start <= 3 { 3 } else { adjusted_start | 1 }; // 确保是奇数
    if odd_start > end {
        return primes;
    }
    
    // 计算奇数数量
    let odd_count = ((end - odd_start) / 2 + 1) as usize;
    
    // 检查内存使用，如果分段太大，使用更小的分段
    let max_safe_size = 100_000_000; // 增加最大安全大小到100MB
    let segment_size = odd_count * std::mem::size_of::<bool>();
    
    if segment_size > max_safe_size {
        // 分段太大，递归处理更小的分段
        let mid = odd_start + (end - odd_start) / 2;
        let left_primes = optimized_linear_sieve_segment_with_small_primes(
            start, mid, small_primes, small_primes_squares
        );
        let right_primes = optimized_linear_sieve_segment_with_small_primes(
            mid + 1, end, small_primes, small_primes_squares
        );
        primes.extend_from_slice(&left_primes);
        primes.extend_from_slice(&right_primes);
        return primes;
    }
    
    let mut is_prime_odd = vec![true; odd_count];
    
    // 使用已知的小素数来筛（只筛奇数）
    // 使用预先计算的平方值，避免重复计算
    for (i, &p) in small_primes.iter().enumerate() {
        if p == 2 {
            continue; // 跳过2，因为我们已经单独处理了偶数
        }
        
        let p_squared = small_primes_squares[i];
        if p_squared > end {
            break;
        }
        
        // 计算第一个大于等于odd_start的p的奇数倍数
        let mut multiple = if odd_start % p == 0 {
            odd_start
        } else {
            odd_start + (p - odd_start % p)
        };
        
        // 确保multiple是奇数
        if multiple % 2 == 0 {
            multiple += p;
        }
        
        // 确保multiple至少是p*p
        if multiple < p_squared {
            multiple = p_squared;
            // 确保p*p是奇数
            if multiple % 2 == 0 {
                multiple += p;
            }
        }
        
        // 标记所有p的奇数倍数为非素数
        while multiple <= end {
            let multiple_index = ((multiple - odd_start) / 2) as usize;
            is_prime_odd[multiple_index] = false;
            
            // 跳过偶数倍数，直接加2p
            multiple += 2 * p;
        }
    }
    
    // 收集素数
    for i in (odd_start..=end).step_by(2) {
        let i_index = ((i - odd_start) / 2) as usize;
        if is_prime_odd[i_index] {
            primes.push(i);
        }
    }
    
    primes
}

// 估算指定范围内的素数数量，使用素数定理
// π(x) ≈ x / ln(x)
fn estimate_prime_count(n: u64) -> usize {
    if n < 2 {
        return 0;
    }
    
    // 使用素数定理估算素数数量
    // 对于大数，使用更精确的近似：x / (ln(x) - 1)
    let estimate = if n > 10000 {
        n as f64 / ((n as f64).ln() - 1.0)
    } else {
        n as f64 / (n as f64).ln()
    };
    
    estimate as usize
}