/// Compute the Scrabble score for a word.
// Letter                           Value
//     A, E, I, O, U, L, N, R, S, T       1
//     D, G                               2
//     B, C, M, P                         3
//     F, H, V, W, Y                      4
//     K                                  5
//     J, X                               8
//     Q, Z                               10
use std::collections::HashMap;

fn setup_values() -> HashMap<char, u8> {
    let mut letter_values: HashMap<char, u8> = HashMap::new();
    let arrangement = vec![
        (1, vec!['A', 'E', 'I', 'O', 'U', 'L', 'N', 'R', 'S', 'T']),
        (2, vec!['D', 'G']),
        (3, vec!['B', 'C', 'M', 'P']),
        (4, vec!['F', 'H', 'V', 'W', 'Y']),
        (5, vec!['K']),
        (8, vec!['J', 'X']),
        (10, vec!['Q', 'Z']),
    ];
    for (value, letters) in arrangement {
        for l in letters {
            letter_values.insert(l, value);
        }
    }

    letter_values
}

pub fn score(word: &str) -> u64 {
    let letter_values = setup_values();
    word.chars()
        .map(|c| c.to_ascii_uppercase())
        .fold(0, |acc, c| match letter_values.get(&c) {
            Some(value) => acc + *value as u64,
            None => acc,
        })
}
