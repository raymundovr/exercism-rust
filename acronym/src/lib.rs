pub fn abbreviate(phrase: &str) -> String {
    let words = phrase
        .split(|c| c == ' ' || c == '-')
        .filter(|word| !word.is_empty());

    let mut abbreviation = String::from("");
    for word in words {
        let w: Vec<char> = word.chars().collect();
        for (i, c) in w.iter().enumerate() {
            if c.is_alphabetic()
                && (i == 0 || (c.is_uppercase() && i > 0 && !w[i - 1].is_uppercase()))
            {
                abbreviation.push(*c);
            }
        }
    }

    abbreviation.to_uppercase()
}
