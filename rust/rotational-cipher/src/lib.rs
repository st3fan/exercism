fn rotate_char(c: char, key: i8) -> char {
    if !c.is_ascii_alphabetic() {
        return c;
    }

    let base = if c.is_uppercase() {
        'A' as i32
    } else {
        'a' as i32
    };

    let c = c as i32 - base;
    let i = (c + key as i32 + 26) % 26;
    ((base as i32 + i) as u8) as char
}

pub fn rotate(input: &str, key: i8) -> String {
    input.chars().map(|c| rotate_char(c, key)).collect()
}
