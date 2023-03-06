// This can probably be done in a much smaller loop but
// this is easy to read and does all the filtering properly.

/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let mut digits = Vec::new();
    for (i, digit) in code
        .chars()
        .filter(|c| !c.is_ascii_whitespace())
        .rev()
        .enumerate()
    {
        eprintln!("Looking at {:?}", digit);

        let Some(digit) = digit.to_digit(10) else {
            return false;
        };

        fn double(n: u32) -> u32 {
            let mut n = n * 2;
            if n > 9 {
                n -= 9;
            }
            n
        }

        if i % 2 == 0 {
            digits.push(digit);
        } else {
            digits.push(double(digit));
        }
    }

    digits.len() > 1 && digits.iter().sum::<u32>() % 10 == 0
}
