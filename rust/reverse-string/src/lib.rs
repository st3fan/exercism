pub fn reverse(input: &str) -> String {
    let mut result = String::new();
    for c in input.chars() {
        result.insert(0, c);
    }
    return result;
}
