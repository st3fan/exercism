pub fn is_armstrong_number(num: u32) -> bool {
    let s = num.to_string();
    let n: u32 = s.to_string()
        .chars()
        .map(|c| (c as u32 - '0' as u32).pow(s.len() as u32))
        .sum();
    n == num
}
