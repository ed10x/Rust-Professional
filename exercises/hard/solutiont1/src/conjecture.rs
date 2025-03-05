pub fn goldbach_conjecture() -> String {
    let primes = sieve_of_eratosthenes(6000);
    let mut result = Vec::with_capacity(2);
    let mut n = 5775;

    while result.len() < 2 {
        n += 2;
        if !can_be_expressed(n, &primes) {
            result.push(n);
        }
    }

    format!("{},{}", result[0], result[1])
}

fn sieve_of_eratosthenes(n: usize) -> Vec<i32> {
    if n < 2 {
        return Vec::new();
    }

    let mut is_prime = vec![1u8; n]; // 使用 u8 代替 bool
    is_prime[0] = 0;
    is_prime[1] = 0;

    let sqrt_n = (n as f64).sqrt() as usize;
    for i in (2..=sqrt_n).step_by(1) {
        if is_prime[i] == 1 {
            for j in (i * i..n).step_by(i) {
                is_prime[j] = 0;
            }
        }
    }

    is_prime.iter()
        .enumerate()
        .filter(|&(_, &is_p)| is_p == 1)
        .map(|(i, _)| i as i32)
        .collect()
}

// 检查一个数是否可以表示为一个素数和一个平方数的两倍之和
fn can_be_expressed(n: i32, primes: &[i32]) -> bool {
    for &p in primes {
        if p >= n {
            break;
        }
        let remainder = n - p;
        if remainder <= 0 {
            continue;
        }

        if remainder % 2 == 0 {
            let half = remainder / 2;
            if is_perfect_square(half) {
                return true;
            }
        }
    }
    false
}

// 判断一个数是否是完全平方数
fn is_perfect_square(x: i32) -> bool {
    if x < 0 {
        return false;
    }
    let sqrt = (x as f64).sqrt() as i32;
    sqrt * sqrt == x
}