/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    let isbn: Vec<char> = isbn.chars().filter(|&c| c != '-').collect();

    if isbn.len() != 10 {
        return false;
    }

    if !isbn[0..9].iter().all(|c| c.is_digit(10)) || !(isbn[9].is_digit(10) || isbn[9] == 'X') {
        return false;
    }

    let mut total: usize = 0;

    for (i, c) in isbn.iter().rev().enumerate() {
        match i {
            0 => {
                if let Some(n) = c.to_digit(10) {
                    total += n as usize;
                } else {
                    total += 10;
                }
            }
            _ => {
                total += (i + 1) * (c.to_digit(10).unwrap() as usize);
            }
        };
    }

    return total % 11 == 0;
}
