#[derive(Debug, PartialEq)]
pub enum Error {
    IncompleteNumber,
    Overflow,
}

fn as_slices(n: &u32) -> Vec<u8> {
    let mut slices: Vec<u8> = Vec::new();
    //least significant
    slices.push(127 & *n as u8);
    //The rest
    let mut shifted: u32 = n >> 7;
    while shifted > 0 {
        slices.insert(0, 128 | (shifted as u8 % 128));
        shifted = shifted >> 7;
    }

    slices
}

/// Convert a list of numbers to a stream of bytes encoded with variable length encoding.
pub fn to_bytes(values: &[u32]) -> Vec<u8> {
    let mut bytes: Vec<u8> = Vec::new();

    for v in values {
        bytes.append(&mut as_slices(v));
    }

    bytes
}

/// Given a stream of bytes, extract all numbers which are encoded in there.
pub fn from_bytes(bytes: &[u8]) -> Result<Vec<u32>, Error> {
    let mut result: Vec<u32> = Vec::new();
    let mut current: u32 = 0;
    for byte in bytes {
        if current.leading_zeros() < 7 {
            return Err(Error::Overflow);
        }

        let number = match *byte < 128 {
            true => *byte,
            false => *byte & 127,
        };

        current = (current << 7) + number as u32;

        if *byte < 128 {
            result.push(current);
            current = 0;
        }
    }

    if result.len() == 0 {
        return Err(Error::IncompleteNumber);
    }

    Ok(result)
}
