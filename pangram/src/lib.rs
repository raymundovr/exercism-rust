/// Determine whether a sentence is a pangram.
use std::collections::HashSet;

pub fn is_pangram(sentence: &str) -> bool {
    let mut letters: HashSet<char> = HashSet::new();

    sentence
        .chars()
        .filter(|c| c.is_ascii() && c.is_alphabetic())
        .map(|c| c.to_ascii_lowercase())
        .for_each(|c| {
            letters.insert(c);
        });

    letters.len() == 26
}
