use std::collections::HashMap;

fn count_words<'a>(words: &'a [&str]) -> HashMap<&'a str, usize> {
    let mut words_with_counts = HashMap::new();
    for word in words.iter() {
        words_with_counts
            .entry(*word)
            .and_modify(|e| *e += 1)
            .or_insert(1);
    }
    words_with_counts
}

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let magazine_words = count_words(magazine);
    let note_words = count_words(note);

    for (&note_word, &note_word_count) in note_words.iter() {
        match magazine_words.get(note_word) {
            None => return false,
            Some(&magazine_word_count) => {
                if magazine_word_count < note_word_count {
                    return false;
                }
            }
        }
    }

    true
}
