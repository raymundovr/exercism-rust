fn normalize(input: &str) -> Vec<char> {
    input
        .chars()
        .filter(|c| c.is_alphanumeric())
        .map(|c| c.to_ascii_lowercase())
        .collect()
}

fn decide_dimensions(len: usize) -> (usize, usize) {
    let len = len as f64;
    let c = len.sqrt().ceil();
    if (c * c) == len {
        return (c as usize, c as usize);
    } else {
        return (c as usize, (c - 1_f64) as usize);
    }
}

fn encode(normalized: Vec<char>, c: usize, r: usize) -> Vec<char> {
    let mut coded: Vec<char> = Vec::new();

    for i in 0..c {
        let mut pos: usize = i;
        for _j in 0..r {
            if pos < normalized.len() {
                coded.push(normalized[pos].clone());
            } else {
                coded.push(' ');
            }
            pos += c;
        }
    }

    coded
}

pub fn encrypt(input: &str) -> String {
    let normalized = normalize(input);
    let (c, r) = decide_dimensions(normalized.len());
    let encoded = encode(normalized, c, r);

    encoded
        .chunks(r)
        .map(|chunk| chunk.into_iter().collect())
        .collect::<Vec<String>>()
        .join(" ")
}
