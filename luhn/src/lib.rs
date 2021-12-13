/// Only digits and spaces
fn is_valid_input(code: &str) -> bool {
    code.trim().len() > 1 &&
    code
        .chars()
        .filter(|&c| !c.is_whitespace() && !c.is_digit(10))
        .count() == 0
}

/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {

    if !is_valid_input(code) {
        return false;
    }

    let sum = code
    .chars()
    .rev()
    .filter(|&x| !x.is_whitespace())
    .map(|ch_number| ch_number.to_digit(10).unwrap())
    .enumerate()
    .map(|(index, value)| {
        match index % 2 {
            1 => {
                let double = value * 2;
                match double >= 10 {
                    true => double - 9,
                    false => double,
                }
            },
            _ => value,
        }
    })
    .sum::<u32>();

    sum % 10 == 0
}
