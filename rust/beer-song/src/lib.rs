pub fn bottle(n: u32) -> String {
    match n {
        1 => "1 bottle".to_string(),
        _ => format!("{} bottles", n)
    }
}

pub fn first(n: u32) -> String {
    match n {
        0 => "No more bottles of beer on the wall, no more bottles of beer.\n".to_string(),
        _ => format!("{} of beer on the wall, {} of beer.\n", bottle(n), bottle(n))
    }
}

pub fn second(n: u32) -> String {
    match n {
        0 => "Go to the store and buy some more, 99 bottles of beer on the wall.\n".to_string(),
        1 => "Take it down and pass it around, no more bottles of beer on the wall.\n".to_string(),
        _ => format!("Take one down and pass it around, {} of beer on the wall.\n", bottle(n-1))
    }
}

pub fn verse(n: u32) -> String {
    first(n) + &second(n)
}

pub fn sing(start: u32, end: u32) -> String {
    let mut result = String::from("");
    for n in (end..start+1).rev() {
        result += &verse(n);
        if n != end {
            result += "\n";
        }
    }
    return result;
}
