use rand::Rng;

fn is_valid_key(key: &str) -> bool {
    !key.is_empty() && key.chars().all(|c| c.is_alphabetic() && c.is_lowercase())
}

pub fn encode_char(k: char, c: char) -> char {
    let k = k as u8 - 'a' as u8;
    let c = c as u8 - 'a' as u8;
    ('a' as u8 + ((c + k) % 26)) as char
}

pub fn decode_char(k: char, c: char) -> char {
    let k = k as u8 - 'a' as u8;
    let c = c as u8 - 'a' as u8;
    ('a' as u8 + ((c + 26 - k) % 26)) as char
}

fn random_key(length: usize) -> String {
    const CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyz";

    let mut rng = rand::thread_rng();
    (0..length)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect::<String>()
}

//

pub fn encode(key: &str, s: &str) -> Option<String> {
    if !is_valid_key(&key) {
        return None;
    }

    key.chars()
        .cycle()
        .zip(s.chars())
        .map(|(k, c)| encode_char(k, c))
        .collect::<String>()
        .into() // This is nicer than wrapping into Some()
}

pub fn decode(key: &str, s: &str) -> Option<String> {
    if !is_valid_key(&key) {
        return None;
    }

    key.chars()
        .cycle()
        .zip(s.chars())
        .map(|(k, c)| decode_char(k, c))
        .collect::<String>()
        .into()
}

pub fn encode_random(s: &str) -> (String, String) {
    let key = random_key(256);
    let enc = encode(key.as_str(), s);
    (key, enc.unwrap())
}
