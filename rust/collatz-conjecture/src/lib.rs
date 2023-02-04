pub fn collatz(mut n: u64) -> Option<u64> {
    if n == 0 || n >= 110243094271 {
        return None;
    }

    let mut steps = 0;
    Some(loop {
        if n == 1 {
            break steps;
        }

        if n % 2 == 0 {
            n = n / 2;
        } else {
            n = (n * 3) + 1;
        }

        steps += 1;
    })
}
