fn swap(input: char) -> Option<char> {
    let d = input.to_ascii_lowercase() as u8;
    match d {
        b'a'..=b'z' => {
            let d: u8 = 97 + (25 - (d - 97));
            Some(d as char)
        }
        b'0'..=b'9' => Some(input),
        _ => None,
    }
}

/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    plain
        .chars()
        .filter_map(|c| swap(c))
        .collect::<Vec<char>>()
        .chunks(5)
        .map(|c| c.into_iter().collect())
        .collect::<Vec<String>>()
        .join(" ")
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    cipher.chars().filter_map(|c| swap(c)).collect()
}
