pub fn raindrops(n: u32) -> String {
    let mut string = String::new();
    if n % 3 == 0 {
        string += "Pling";
    }
    if n % 5 == 0 {
        string += "Plang";
    }
    if n % 7 == 0 {
        string += "Plong";
    }
    if string == "" {
        string = n.to_string();
    }
    return string;
}
