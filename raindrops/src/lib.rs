pub fn raindrops(n: u32) -> String {
    let mut result = String::new();
    let drops = vec![(3, "Pling"), (5, "Plang"), (7, "Plong")];
    for (factor, drop) in drops.iter() {
        if n % factor == 0 {
            result.push_str(drop);
        }
    }

    if result.is_empty() {
        return n.to_string();
    }

    result
}
