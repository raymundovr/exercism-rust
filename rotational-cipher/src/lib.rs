fn get_rotated(c: char, key: i8) -> char {
    let d = c as u8;
    let key = key % 26;
    println!("{}: {} ->", c, d);
    let rotated = match d {
        b'a'..=b'z' => 97 + (((d - 97) as i8 + key) % 26) as u8,
        b'A'..=b'Z' => 65 + (((d - 65) as i8 + key) % 26) as u8,
        _ => d,
    };

    rotated as char
}

pub fn rotate(input: &str, key: i8) -> String {
    input.chars().map(|c| get_rotated(c, key)).collect()
}
