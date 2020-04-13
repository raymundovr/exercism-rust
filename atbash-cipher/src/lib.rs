fn swap(input: char) -> char {
    let d = input as u8;
    let output = match d >= 97 && d <= 122 {
        true => 97 + (25 - (d - 97)),
        false => d,
    };

    output as char
}

/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    plain
        .to_lowercase()
        .chars()
        .filter(|c| c.is_alphanumeric())
        .map(|c| swap(c))
        .collect::<Vec<char>>()
        .chunks(5)
        .map(|c| c.into_iter().collect())
        .collect::<Vec<String>>()
        .join(" ")
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    cipher.replace(" ", "").chars().map(|c| swap(c)).collect()
}
