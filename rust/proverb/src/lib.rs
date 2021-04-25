pub fn build_proverb(list: &[&str]) -> String {
    let mut result = String::new();
    if list.len() != 0 {
        for i in 0..(list.len()-1) {
            result += &format!("For want of a {} the {} was lost.\n", list[i], list[i+1]);
        }
        result += &format!("And all for the want of a {}.", list[0]);
    }
    return result;
}
