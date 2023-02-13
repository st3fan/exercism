use regex::Regex;

// TODO Is there a way to create the regular expressions as globals/consts without depending on an external crate?

fn split_phrase(phrase: &str) -> Vec<String> {
    let re = Regex::new(r"([A-Za-z]+)(?:'[A-Za-z]+)*").expect("Invalid regex");
    re.captures_iter(phrase)
        .map(|c| String::from(c[1].to_string()))
        .collect::<Vec<String>>()
}

fn split_word(word: &str) -> Vec<String> {
    let re = Regex::new(r"([A-Z]*[a-z]*)").expect("Invalid regex");
    re.find_iter(word)
        .map(|s| String::from(s.as_str()))
        .collect::<Vec<String>>()
}

pub fn abbreviate(phrase: &str) -> String {
    let mut acronym = String::new();
    for word in split_phrase(phrase) {
        for word in split_word(word.as_str()) {
            let c = word.chars().next().unwrap().to_ascii_uppercase();
            acronym.push(c);
        }
    }
    acronym
}
