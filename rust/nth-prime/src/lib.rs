pub fn is_prime(n: u32) -> bool {
    for d in 2..(n / 2 + 1) {
        if n % d == 0 {
            return false;
        }
    }
    true
}

pub fn nth(n: u32) -> u32 {
    let mut p = 2;
    let mut i = 0;
    loop {
        if is_prime(p) {
            if i == n {
                return p
            } else {
                i += 1;
            }
        }
        p += 1;
    }
}
