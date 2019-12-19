use primes::PrimeSet;

pub fn factors(n: u64) -> Vec<u64> {
    let mut factors: Vec<u64> = Vec::new();

    if n < 2 {
        return factors;
    }

    let mut primes = PrimeSet::new();
    let mut prime_iter = primes.iter();
    let mut residual: u64 = n;
    let mut prime = prime_iter.next().unwrap();
    while residual > 1 {
        if residual % prime == 0 {
            factors.push(prime);
            residual = residual / prime;
        } else {
            prime = prime_iter.next().unwrap();
        }
    }

    factors
}
