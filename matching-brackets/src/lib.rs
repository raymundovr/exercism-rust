use std::collections::HashMap;
use std::collections::VecDeque;

pub fn brackets_are_balanced(string: &str) -> bool {
    let mut queue: VecDeque<char> = VecDeque::new();
    let brackets: HashMap<char, char> = [('{', '}'), ('[', ']'), ('(', ')')]
        .iter()
        .cloned()
        .collect();
    let is_opening = |c| brackets.contains_key(&c);
    let is_closing = |c| brackets.values().position(|&v| v == c).is_some();

    for c in string.chars() {
        if is_opening(c) {
            queue.push_front(c);
        } else if is_closing(c) {
            if queue.is_empty() {
                return false;
            }
            let last_bracket = queue.pop_front().unwrap();
            if c != *brackets.get(&last_bracket).unwrap() {
                return false;
            }
        }
    }

    queue.is_empty()
}
