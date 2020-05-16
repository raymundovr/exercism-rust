use std::collections::HashSet;

fn normalize(word: &str) -> Vec<char> {
    let word = word.to_lowercase();
    let mut normalized: Vec<char> = word.chars().collect();
    normalized.sort();

    normalized
}

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let normalized_word = normalize(word);
    let word = word.to_lowercase();

    possible_anagrams
        .into_iter()
        .map(|p| p.to_owned())
        .filter(|p| p.to_lowercase() != word && normalize(p) == normalized_word)
        .collect::<HashSet<&str>>()
}
