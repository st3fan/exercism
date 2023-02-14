/// While the problem description indicates a return status of 1 should be returned on errors,
/// it is much more common to return a `Result`, so we provide an error type for the result here.
#[derive(Debug, Eq, PartialEq)]
pub enum AffineCipherError {
    NotCoprime(i32),
}

fn mmi(a: i32, m: i32) -> Result<i32, AffineCipherError> {
    for x in 1..m {
        if ((a % m) * (x % m)) % m == 1 {
            return Ok(x);
        }
    }
    Err(AffineCipherError::NotCoprime(a))
}

#[test]
fn test_mmi() {
    assert_eq!(mmi(9, 26).unwrap(), 3);
    assert_eq!(mmi(15, 26).unwrap(), 7);
}

fn encrypt(c: char, a: i32, b: i32) -> char {
    if c.is_ascii_digit() {
        return c;
    }
    let c = c as i32 - 'a' as i32;
    let i = (a * c + b) % 26;
    ((i + 'a' as i32) as u8) as char
}

fn decrypt(c: char, mmi: i32, b: i32) -> char {
    if c.is_ascii_digit() {
        return c;
    }
    let c = c as i32 - 'a' as i32;
    let i = (((mmi * (c - b)) % 26) + 26) % 26; // Fun one - saves a conditional
    (('a' as i32 + i) as u8) as char
}

const CIPHERTEXT_CHUNK_SIZE: usize = 5;

/// Encodes the plaintext using the affine cipher with key (`a`, `b`). Note that, rather than
/// returning a return code, the more common convention in Rust is to return a `Result`.
pub fn encode(plaintext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    let _ = mmi(a, 26)?; // TODO This looks weird

    let ciphertext: String = plaintext
        .chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .map(|c| c.to_ascii_lowercase())
        .map(|c| encrypt(c, a, b))
        .collect();

    let result = ciphertext
        .chars()
        .collect::<Vec<char>>()
        .chunks(CIPHERTEXT_CHUNK_SIZE)
        .map(|c| c.iter().collect::<String>())
        .collect::<Vec<String>>()
        .join(" ");

    Ok(result)
}

/// Decodes the ciphertext using the affine cipher with key (`a`, `b`). Note that, rather than
/// returning a return code, the more common convention in Rust is to return a `Result`.
pub fn decode(ciphertext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    let mmi = mmi(a, 26)?;

    Ok(ciphertext
        .chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .map(|c| decrypt(c, mmi, b))
        .collect::<String>())
}
