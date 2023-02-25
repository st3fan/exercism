use std::collections::HashSet;

/// Determine whether a sentence is a pangram.
pub fn is_pangram(sentence: &str) -> bool {
    sentence
        .chars()
        .filter(|c| c.is_ascii_alphabetic())
        .map(|c| c.to_ascii_lowercase())
        .collect::<HashSet<char>>()
        .len()
        == 26
}

// I prefer this one though, for clarity.

// pub fn is_pangram(sentence: &str) -> bool {
//     let letters: HashSet<char> = sentence
//         .chars()
//         .filter(|c| c.is_ascii_alphabetic())
//         .map(|c| c.to_ascii_lowercase())
//         .collect();
//     letters.len() == 26
// }
