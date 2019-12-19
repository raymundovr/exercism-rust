fn is_factor(x: u32, factors: &[u32]) -> bool {
    factors.iter().any(|f| f > &0 && x % f == 0)
}

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    (1..limit).filter(|x| is_factor(*x, factors)).sum()
}
