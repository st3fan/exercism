fn rotate_char(c: char) -> char {
    if c.is_ascii_digit() {
        return c;
    }
    let c = 'z' as i32 - (c as i32 - 'a' as i32);
    (c as u8) as char
}

const CIPHERTEXT_CHUNK_SIZE: usize = 5;

/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    let encoded: String = plain
        .chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .map(|c| c.to_ascii_lowercase())
        .map(|c| rotate_char(c))
        .collect();

    // TODO Do we really have to do this in two steps?
    encoded
        .chars()
        .collect::<Vec<char>>()
        .chunks(CIPHERTEXT_CHUNK_SIZE)
        .map(|c| c.iter().collect::<String>())
        .collect::<Vec<String>>()
        .join(" ")
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    cipher
        .chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .map(|c| c.to_ascii_lowercase())
        .map(|c| rotate_char(c))
        .collect()
}
