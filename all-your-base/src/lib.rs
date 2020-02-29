#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    if from_base < 2 {
        return Err(Error::InvalidInputBase);
    }

    if to_base < 2 {
        return Err(Error::InvalidOutputBase);
    }

    if let Some(&d) = number.iter().find(|&n| *n >= from_base) {
        return Err(Error::InvalidDigit(d));
    }

    let value = number
        .iter()
        .rev()
        .enumerate()
        .fold(0, |acc, (i, &n)| acc + n * from_base.pow(i as u32));

    Ok(val_to_base(value, to_base))
}

fn val_to_base(value: u32, to_base: u32) -> Vec<u32> {
    let mut result: Vec<u32> = Vec::new();
    let mut residual = value;

    while residual > 0 {
        result.insert(0, residual % to_base);
        residual /= to_base;
    }
    result
}
