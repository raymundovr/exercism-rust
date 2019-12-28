use std::collections::HashSet;

pub fn check(candidate: &str) -> bool {
    let mut letters = HashSet::new();
    candidate
        .chars()
        .filter(|c| *c != ' ' && *c != '-')
        .map(|c| c.to_lowercase().to_string())
        .fold(true, |acc, l| {
            let is_new = !letters.contains(&l);
            letters.insert(l);
            acc && is_new
        })
}
