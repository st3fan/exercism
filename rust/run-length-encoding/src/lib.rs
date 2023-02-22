// Not very proud of both of these ...

pub fn encode(source: &str) -> String {
    let mut encoded = String::new();

    let mut count = 0;
    let mut current = 0 as char;

    for c in source.chars() {
        if c != current {
            if count != 0 {
                if count != 1 {
                    encoded.push_str(count.to_string().as_str());
                }
                encoded.push(current);
            }
            count = 1;
            current = c;
        } else {
            count += 1;
        }
    }

    if count != 0 {
        if count != 1 {
            encoded.push_str(count.to_string().as_str());
        }
        encoded.push(current);
    }

    encoded
}

pub fn decode(source: &str) -> String {
    let mut decoded = String::new();

    let mut chars = source.chars().peekable();

    loop {
        if let Some(c) = chars.next() {
            if c.is_numeric() {
                let mut n = String::new();
                n.push(c);

                while let Some(c) = chars.peek() {
                    if !c.is_numeric() {
                        break;
                    }
                    n.push(chars.next().unwrap());
                }

                let c = chars.next().unwrap();
                let n = n.parse::<i32>().unwrap();
                for _ in 0..n {
                    decoded.push(c);
                }
            } else {
                decoded.push(c);
            }
        } else {
            break;
        }
    }
    decoded
}
