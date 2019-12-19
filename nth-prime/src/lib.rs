fn is_prime(n: u32) -> bool {
    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }
    let mut i = 5;
    while (i * i) <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i = i + 6;
    }
    true
}

pub fn nth(n: u32) -> u32 {
    let mut primes: Vec<u32> = vec![2, 3];
    let mut i = 5;
    while (primes.len() as u32) <= n {
        if is_prime(i) {
            primes.push(i);
        }
        i = i + 2;
    }

    primes.pop().unwrap()
}
