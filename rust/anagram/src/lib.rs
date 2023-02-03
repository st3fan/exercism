use std::collections::HashSet;
use unicode_segmentation::UnicodeSegmentation;

trait Anagram {
    fn is_anagram_of(self, possible_anagram: Self) -> bool;
}

impl Anagram for String {
    fn is_anagram_of(self, possible_anagram: Self) -> bool {
        if self.to_lowercase() == possible_anagram.to_lowercase() {
            return false;
        }

        let a = self.to_lowercase();
        let mut a: Vec<&str> = a.graphemes(true).collect();
        a.sort();

        let b = possible_anagram.to_lowercase();
        let mut b: Vec<&str> = b.graphemes(true).collect();
        b.sort();

        a == b
    }
}

impl Anagram for &str {
    fn is_anagram_of(self, possible_anagram: Self) -> bool {
        let word = String::from(self);
        word.is_anagram_of(String::from(possible_anagram))
    }
}

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let anagrams: HashSet<&str> = possible_anagrams
        .iter()
        .cloned()
        .filter(|anagram| word.is_anagram_of(*anagram))
        .collect();
    anagrams
}
