fn is_prime(n :u32) -> bool {
    if n <= 3 {
        return n > 1;
    } else if n % 2 == 0 || n % 3 == 0 {
        return false;
    }

    let mut i = 5;

    while i*i <= n {
        if n % i == 0 || n % (i+2) == 0 {
            return false;
        }
        i = i + 6;
    }

    return true
}

pub fn nth(mut n: u32) -> u32 {
    let mut v = 2;
    loop {
        if is_prime(v) {
            if n == 0 {
                return v;
            }
            n -= 1;
        }
        v += 1;
    }
}
