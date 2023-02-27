#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    SpanTooLong,
    InvalidDigit(char),
}

pub fn lsp(string_digits: &str, span: usize) -> Result<u64, Error> {
    let mut digits = Vec::new();
    for c in string_digits.chars() {
        let Some(digit) = c.to_digit(10) else {
            return Err(Error::InvalidDigit(c));
        };
        digits.push(digit as u64);
    }

    if span > digits.len() {
        return Err(Error::SpanTooLong);
    }

    if span == 0 {
        return Ok(1);
    }

    if digits.is_empty() {
        return Err(Error::SpanTooLong);
    }

    let mut max: u64 = 0;
    for window in digits.windows(span) {
        let v: u64 = window.iter().product();
        if v > max {
            max = v;
        }
    }

    Ok(max)
}
