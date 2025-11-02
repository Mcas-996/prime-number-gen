use std::time::Instant;
use std::io::{self, Write};
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use std::collections::HashMap;

fn main(){
    println!("Generating 10 prime numbers with 64 bits using linear sieve...");
    
    // 提示用户输入种子
    print!("Enter a seed number (or press Enter for a random seed): ");
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let input = input.trim();
    
    // 根据用户输入决定种子
    let seed = if input.is_empty() {
        // 如果用户没有输入，使用Windows API生成真随机数
        generate_windows_random_seed()
    } else {
        // 使用用户输入的数字作为种子
        input.parse::<u64>().unwrap_or_else(|_| {
            println!("Invalid input, using default seed 42");
            42
        })
    };
    
    println!("Using seed: {}", seed);
    let start_time = Instant::now();
    
    let primes = generate_large_primes_with_sieve(10, 64, seed);
    
    let duration = start_time.elapsed();
    
    println!("\nGenerated {} prime numbers in {:.2} seconds:", primes.len(), duration.as_secs_f64());
    for (i, prime) in primes.iter().enumerate() {
        println!("Prime {}: {}", i + 1, prime);
    }
}

// 使用Windows API生成真随机数种子
fn generate_windows_random_seed() -> u64 {
    // 在Windows上，我们使用thread_rng()获取高质量的随机数
    // 这在底层会调用Windows的CryptGenRandom或BCryptGenRandom API
    let mut rng = rand::thread_rng();
    rng.gen::<u64>()
}





// Miller-Rabin素性测试
fn is_miller_rabin_prime(n: u128) -> bool {
    if n < 2 {
        return false;
    }
    
    // 处理偶数
    if n % 2 == 0 {
        return n == 2;
    }
    
    // 将n-1表示为d*2^s
    let mut d = n - 1;
    let mut s = 0;
    while d % 2 == 0 {
        d /= 2;
        s += 1;
    }
    
    // 测试基数
    let bases = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37];
    
    for &a in &bases {
        if a >= n {
            continue;
        }
        
        if !miller_rabin_test(n, a, d, s) {
            return false;
        }
    }
    
    true
}

// Miller-Rabin测试的单次测试
fn miller_rabin_test(n: u128, a: u128, d: u128, s: usize) -> bool {
    let mut x = mod_pow(a, d, n);
    
    if x == 1 || x == n - 1 {
        return true;
    }
    
    for _ in 0..s - 1 {
        x = mod_pow(x, 2, n);
        if x == n - 1 {
            return true;
        }
    }
    
    false
}

// 模幂运算
fn mod_pow(mut base: u128, mut exp: u128, modulus: u128) -> u128 {
    if modulus == 1 {
        return 0;
    }
    
    let mut result = 1;
    base %= modulus;
    
    while exp > 0 {
        if exp % 2 == 1 {
            result = (result * base) % modulus;
        }
        exp >>= 1;
        base = (base * base) % modulus;
    }
    
    result
}



// 使用线性筛法生成指定位数和大小的素数
fn generate_large_primes_with_sieve(count: usize, bit_size: usize, seed: u64) -> Vec<u128> {
    let mut primes = Vec::new();
    
    // 使用用户提供的种子创建随机数生成器
    let mut rng = StdRng::seed_from_u64(seed);
    
    // 计算指定位数的最小值和最大值
    let min_value = 1u128 << (bit_size - 1);
    let max_value = if bit_size == 128 { u128::MAX } else { (1u128 << bit_size) - 1 };
    
    // 使用线性筛法生成小素数，用于测试大数的素性
    print!("Generating small primes up to 1,000,000... ");
    io::stdout().flush().unwrap();
    let small_primes = generate_small_primes(1000000); // 生成100万以内的素数
    println!("Done. Found {} small primes.", small_primes.len());
    
    // 使用随机采样代替顺序检查，提高效率
    let mut attempts = 0;
    let max_attempts = count * 100000; // 增加尝试次数限制到count*100000次
    
    println!("Searching for primes... (Max attempts: {})", max_attempts);
    
    while primes.len() < count && attempts < max_attempts {
        // 生成指定位数的随机数
        let mut candidate = rng.gen_range(min_value..=max_value);
        
        // 确保是奇数（除了2，所有素数都是奇数）
        if candidate % 2 == 0 {
            candidate += 1;
        }
        
        attempts += 1;
        
        // 更新进度条 - 每1000次尝试更新一次
        if attempts % 1000 == 0 || primes.len() > 0 {
            let progress = (attempts * 100) / max_attempts;
            print_progress_bar(progress as usize, primes.len(), count);
        }
        
        // 使用线性筛法测试素性
        if is_prime_with_sieve(candidate, &small_primes) {
            primes.push(candidate);
            println!("\nFound prime #{}: {}", primes.len(), candidate);
        }
    }
    
    // 最后更新一次进度条
    let progress = (attempts * 100) / max_attempts;
    print_progress_bar(progress as usize, primes.len(), count);
    println!();
    
    primes
}

// 显示进度条
fn print_progress_bar(progress: usize, found: usize, target: usize) {
    print!("\r[");
    for i in 0..50 {
        if i < progress / 2 {
            print!("=");
        } else if i == progress / 2 {
            print!(">");
        } else {
            print!(" ");
        }
    }
    print!("] {}% (Found {}/{})", progress, found, target);
    io::stdout().flush().unwrap();
}

// 使用线性筛法生成小素数
fn generate_small_primes(limit: usize) -> Vec<u128> {
    let mut is_prime = vec![true; limit + 1];
    is_prime[0] = false;
    is_prime[1] = false;
    
    let mut primes = Vec::new();
    
    for i in 2..=limit {
        if is_prime[i] {
            primes.push(i as u128);
            for j in ((i * i)..=limit).step_by(i) {
                is_prime[j] = false;
            }
        }
    }
    
    primes
}

// 使用小素数列表测试大数的素性
fn is_prime_with_sieve(n: u128, small_primes: &[u128]) -> bool {
    // 处理小数字
    if n <= 1 {
        return false;
    }
    if n <= 3 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }
    
    // 首先检查是否在小素数列表中
    if small_primes.contains(&n) {
        return true;
    }
    
    // 使用小素数测试，优化：提前计算平方根避免重复计算
    let sqrt_n = (n as f64).sqrt() as u128;
    for &p in small_primes {
        if p > sqrt_n {
            break;
        }
        if n % p == 0 {
            return false;
        }
    }
    
    // 通过小素数测试后，使用Miller-Rabin测试
    is_miller_rabin_prime(n)
}