#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    IncompleteNumber,
    Overflow,
}

/// Convert a list of numbers to a stream of bytes encoded with
/// variablelength encoding.
pub fn to_bytes(values: &[u32]) -> Vec<u8> {
    let mut encoded: Vec<u8> = Vec::new();
    for &value in values.iter() {
        if value < 0x00000080 {
            encoded.push(value as u8);
            continue;
        }

        if value < 0x00004000 {
            encoded.push(((value >> 7) as u8 & 0b01111111) | 0b10000000);
            encoded.push(value as u8 & 0b01111111);
            continue;
        }

        if value < 0x00200000 {
            encoded.push(((value >> 14) as u8 & 0b01111111) | 0b10000000);
            encoded.push(((value >> 7) as u8 & 0b01111111) | 0b10000000);
            encoded.push((value >> 0) as u8 & 0b01111111);
            continue;
        }

        if value < 0x10000000 {
            encoded.push(((value >> 21) as u8 & 0b01111111) | 0b10000000);
            encoded.push(((value >> 14) as u8 & 0b01111111) | 0b10000000);
            encoded.push(((value >> 7) as u8 & 0b01111111) | 0b10000000);
            encoded.push((value >> 0) as u8 & 0b01111111);
            continue;
        }

        encoded.push(((value >> 28) as u8 & 0b01111111) | 0b10000000);
        encoded.push(((value >> 21) as u8 & 0b01111111) | 0b10000000);
        encoded.push(((value >> 14) as u8 & 0b01111111) | 0b10000000);
        encoded.push(((value >> 7) as u8 & 0b01111111) | 0b10000000);
        encoded.push((value >> 0) as u8 & 0b01111111);
    }
    encoded
}

/// Given a stream of bytes, extract all numbers which are encoded
/// in there.
pub fn from_bytes(bytes: &[u8]) -> Result<Vec<u32>, Error> {
    let mut decoded = Vec::new();
    let mut bytes = bytes.iter().peekable();

    while bytes.peek() != None {
        let b = *bytes.next().unwrap();
        let mut value: u32 = (b & 0b01111111) as u32;

        while b & 0b10000000 != 0 {
            if let Some(b) = bytes.next() {
                if value & 0b11111110_00000000_00000000_00000000 != 0 {
                    return Err(Error::Overflow);
                };

                value = (value << 7) | (b & 0b01111111) as u32;

                if b & 0b10000000 == 0 {
                    break;
                }
            } else {
                return Err(Error::IncompleteNumber);
            }
        }

        decoded.push(value);
    }

    // 1111 1111111 1111111 1111111 1111111

    Ok(decoded)
}
