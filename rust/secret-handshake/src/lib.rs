pub fn actions(n: u8) -> Vec<&'static str> {
    let mut result = Vec::new();
    if n & 0b00000001 != 0 {
        result.push("wink");
    }
    if n & 0b00000010 != 0 {
        result.push("double blink");
    }
    if n & 0b00000100 != 0 {
        result.push("close your eyes");
    }
    if n & 0b00001000 != 0 {
        result.push("jump");
    }
    if n & 0b00010000 != 0 {
        result.reverse();
    }
    result
}
