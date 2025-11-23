use std::time::Instant;

/// Rough estimate for the number of primes up to x.
pub fn estimate_pi(x: f64) -> u64 {
    if x < 2.0 {
        return 0;
    }
    let x_ln = x.ln();
    if x >= 55.0 {
        (x / (x_ln - 1.0)) as u64
    } else {
        (x / x_ln) as u64
    }
}

/// Simple sieve to generate primes up to `limit`.
pub fn precompute_small_primes(limit: u64) -> Vec<u64> {
    let limit = limit as usize;
    if limit < 2 {
        return Vec::new();
    }

    let mut is_prime = vec![true; limit + 1];
    is_prime[0] = false;
    is_prime[1] = false;

    if limit >= 2 {
        is_prime[2] = true;
        for i in (4..=limit).step_by(2) {
            is_prime[i] = false;
        }
    }

    for i in (3..=(limit as f64).sqrt() as usize).step_by(2) {
        if is_prime[i] {
            for j in (i * i..=limit).step_by(i * 2) {
                is_prime[j] = false;
            }
        }
    }

    let mut primes = Vec::new();
    for i in 2..=limit {
        if is_prime[i] {
            primes.push(i as u64);
        }
    }
    primes
}

/// Segmented sieve with odd-only marking to reduce memory.
pub fn segmented_sieve(max_value: u64, _batch_size: u64) -> Vec<u64> {
    let start = Instant::now();

    let sqrt_max = (max_value as f64).sqrt() as u64 + 1;
    println!(
        "Precomputing primes up to sqrt({}) = {}...",
        max_value, sqrt_max
    );
    let base_primes = precompute_small_primes(sqrt_max);
    println!(
        "Found {} base primes, took {:?} ms",
        base_primes.len(),
        start.elapsed().as_millis()
    );

    let mut all_primes = Vec::with_capacity(estimate_pi(max_value as f64) as usize);

    if max_value >= 2 {
        all_primes.push(2);
    }

    const SEGMENT_SIZE: u64 = 1_000_000;

    let segment_count = if max_value <= 2 {
        0
    } else {
        ((max_value - 2 + SEGMENT_SIZE - 1) / SEGMENT_SIZE) as usize
    };
    let current_time = Instant::now();
    let mut total_primes_found = if max_value >= 2 { 1 } else { 0 };

    println!(
        "Starting segmented processing, total {} segments",
        segment_count
    );

    for segment_idx in 0..segment_count {
        let segment_start = 2 + segment_idx as u64 * SEGMENT_SIZE;
        let segment_end = std::cmp::min(segment_start + SEGMENT_SIZE - 1, max_value);

        if segment_start > segment_end {
            break;
        }

        let segment_size = (segment_end - segment_start + 1) as usize;
        let mut is_prime_segment = vec![true; segment_size];

        for &prime in &base_primes {
            if prime == 2 {
                continue;
            }

            let prime_squared = match prime.checked_mul(prime) {
                Some(val) => val,
                None => continue,
            };

            if prime_squared > segment_end {
                continue;
            }

            let mut start_multiple = if prime_squared >= segment_start {
                prime_squared
            } else {
                let remainder = segment_start % prime;
                if remainder == 0 {
                    segment_start
                } else {
                    segment_start + (prime - remainder)
                }
            };

            // We later step by 2 * prime to skip even multiples, so ensure we start from an odd multiple.
            if start_multiple % 2 == 0 {
                match start_multiple.checked_add(prime) {
                    Some(next) if next <= segment_end => start_multiple = next,
                    _ => continue, // No odd multiple of `prime` exists in this segment (or overflowed).
                }
            }

            let mut multiple = start_multiple;
            while multiple <= segment_end && multiple >= segment_start {
                let index = (multiple - segment_start) as usize;
                if index < segment_size {
                    is_prime_segment[index] = false;
                }

                match multiple.checked_add(2 * prime) {
                    Some(next) => multiple = next,
                    None => break,
                }
            }
        }

        for (i, &prime_flag) in is_prime_segment.iter().enumerate() {
            if prime_flag {
                let num = segment_start + i as u64;
                if num > 2 && num % 2 == 1 {
                    all_primes.push(num);
                    total_primes_found += 1;
                }
            }
        }

        if segment_idx % 10 == 0 || segment_idx == segment_count - 1 {
            let progress_percent = (segment_idx as f64 / segment_count as f64) * 100.0;
            let elapsed = current_time.elapsed().as_secs_f64();
            let rate = if elapsed > 0.0 {
                (segment_idx as f64 * SEGMENT_SIZE as f64) / elapsed
            } else {
                0.0
            };

            println!(
                "[{}] {:.1}% {}M {:.0}n/s segment {} base_primes {} primes_found {}",
                "=".repeat((progress_percent as usize / 10).min(10)),
                progress_percent,
                (segment_idx as u64 * SEGMENT_SIZE) / 1_000_000,
                rate,
                segment_idx,
                base_primes.len(),
                total_primes_found
            );
        }
    }

    all_primes
}
