use std::collections::HashSet;

pub fn check(candidate: &str) -> bool {
    let letters: Vec<char> = candidate
        .chars()
        .filter(|c| c.is_ascii_alphabetic())
        .map(|c| c.to_ascii_lowercase())
        .collect();
    letters.len() == letters.into_iter().collect::<HashSet<char>>().len()
}
