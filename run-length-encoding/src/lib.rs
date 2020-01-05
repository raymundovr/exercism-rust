fn compress(count: u8, c: char) -> String {
    if count > 1 {
        return String::from(format!("{}{}", count, c));
    }

    String::from(c.to_string())
}

pub fn encode(source: &str) -> String {
    let mut encoded = String::new();
    if source.len() > 0 {
        let mut chars = source.chars().peekable();
        let mut current = chars.next().unwrap();
        let mut current_count: u8 = 1;
        while chars.peek().is_some() {
            let next = chars.next().unwrap();
            if current != next {
                encoded.push_str(&compress(current_count, current));
                current = next;
                current_count = 1;
            } else {
                current_count += 1;
            }
        }

        encoded.push_str(&compress(current_count, current));
    }

    encoded
}

pub fn decode(source: &str) -> String {
    let mut decoded = String::new();
    let mut chars = source.chars().fuse();
    let mut count = String::new();

    while let Some(c) = chars.next() {
        if c.is_digit(10) {
            count.push(c);
        } else {
            let ucount = match count.parse::<usize>() {
                Ok(n) => n,
                _ => 1,
            };

            decoded.push_str(&c.to_string().repeat(ucount));
            count = String::new();
        }
    }

    decoded
}
