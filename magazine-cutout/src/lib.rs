// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut dictionary: HashMap<String, u8> = HashMap::new();

    for &word in magazine {
        let counter = dictionary.entry(word.to_string()).or_insert(0);
        *counter += 1;
    }

    for &word in note {
        match dictionary.get_mut(&word.to_string()) {
            None => return false,
            Some(count) => {
                if *count == 0 {
                    return false;
                }
                *count -= 1;
            }
        }
    }

    true
}
