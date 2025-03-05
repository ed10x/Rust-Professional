pub fn find_max_prime_factor(mut n: u128) -> u128 {
    if n == 0 {
        return 0;
    }
    let mut max_prime = 1;

    if n % 2 == 0 {
        max_prime = 2;
        while n % 2 == 0 {
            n /= 2;
        }
        if n == 1 {
            return max_prime;
        }
        if is_prime(n) {
            return max_prime.max(n);
        }
    }

    let mut i = 3;
    while i <= n / i {
        if n % i == 0 {
            max_prime = i;
            while n % i == 0 {
                n /= i;
            }
            if n == 1 {
                return max_prime;
            }
            if is_prime(n) {
                return max_prime.max(n);
            }
        }
        i += 2;
    }

    if n > 1 {
        max_prime = max_prime.max(n);
    }

    max_prime
}

fn is_prime(n: u128) -> bool {
    if n <= 1 {
        return false;
    } else if n <= 3 {
        return true;
    } else if n % 2 == 0 {
        return false;
    }

    let mut d = n - 1;
    let mut s = 0;
    while d % 2 == 0 {
        d /= 2;
        s += 1;
    }

    let bases = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37];
    for &a in &bases {
        if a >= n {
            continue;
        }
        let mut x = mod_pow(a, d, n);
        if x == 1 || x == n - 1 {
            continue;
        }
        let mut found = false;
        for _ in 0..s - 1 {
            x = mod_pow(x, 2, n);
            if x == n - 1 {
                found = true;
                break;
            }
        }
        if !found {
            return false;
        }
    }
    true
}

fn mod_pow(mut base: u128, mut exp: u128, modulus: u128) -> u128 {
    let mut result = 1;
    base %= modulus;
    while exp > 0 {
        if exp % 2 == 1 {
            result = mul_mod(result, base, modulus);
        }
        base = mul_mod(base, base, modulus);
        exp >>= 1;
    }
    result
}

fn mul_mod(a: u128, b: u128, m: u128) -> u128 {
    let mut a = a % m;
    let mut b = b % m;
    let mut res = 0;
    while b > 0 {
        if b % 2 == 1 {
            res = (res + a) % m;
        }
        a = (a * 2) % m;
        b /= 2;
    }
    res
}