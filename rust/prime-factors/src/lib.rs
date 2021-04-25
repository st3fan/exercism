
pub fn factors(n: u64) -> Vec<u64> {
    let mut v = n;
    let mut f = 2;
    let mut result = vec![];

    while v > 1 {
        if v % f == 0 {
            result.push(f);
            v = v / f;
        } else {
            f += 1;
        }
    }

    return result;
}
