use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    let mut result: BTreeMap<char, i32> = BTreeMap::new();
    for (score, letters) in h {
        for letter in letters {
            result.insert(letter.to_ascii_lowercase(), *score);
        }
    }
    result
}
