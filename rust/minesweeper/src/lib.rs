// This. Is. All. Horrible. Code.

struct Position {
    x: i32,
    y: i32,
}

// This. Is. All. Horrible. Code.

fn positions(x: i32, y: i32, w: i32, h: i32) -> Vec<Position> {
    let mut positions = Vec::new();

    for dx in -1..=1 {
        for dy in -1..=1 {
            if dx == 0 && dy == 0 {
                continue;
            }
            let x = x + dx;
            let y = y + dy;

            if x >= 0 && x < w && y >= 0 && y < h {
                positions.push(Position { x, y });
            }
        }
    }

    positions
}

// This. Is. All. Horrible. Code.

fn to_count(n: usize) -> u8 {
    match n {
        0 => '0' as u8,
        1 => '1' as u8,
        2 => '2' as u8,
        3 => '3' as u8,
        4 => '4' as u8,
        5 => '5' as u8,
        6 => '6' as u8,
        7 => '7' as u8,
        8 => '8' as u8,
        9 => '9' as u8,
        _ => panic!("Invalid count"),
    }
}

// This. Is. All. Horrible. Code.

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let mut result = Vec::new();

    for &line in minefield {
        result.push(String::from(line));
    }

    if minefield.len() == 0 || minefield[0].len() == 0 {
        return result;
    }

    let w = minefield[0].len() as i32;
    let h = minefield.len() as i32;

    for x in 0..result[0].len() as i32 {
        for y in 0..result.len() as i32 {
            if result[y as usize].as_bytes()[x as usize] == ('*' as u8) {
                continue;
            }

            let mut n = 0;
            for p in positions(x, y, w, h) {
                if result[p.y as usize].as_bytes()[p.x as usize] == ('*' as u8) {
                    n += 1;
                }
            }
            if n != 0 {
                let b = unsafe { result[y as usize].as_bytes_mut() };
                b[x as usize] = to_count(n);
            }
        }
    }

    result
}
