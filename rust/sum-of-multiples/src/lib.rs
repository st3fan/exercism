fn has_factor(n: u32, factors: &[u32]) -> Option<u32> {
    if factors.iter().any(|&x| x > 0 && n % x == 0) {
        Some(n)
    } else {
        None
    }
}

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    (1..limit).filter_map(|x| has_factor(x, factors)).sum()
}
