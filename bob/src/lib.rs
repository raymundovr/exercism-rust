fn is_question(message: &str) -> bool {
    message.chars().last().unwrap() == '?'
}

fn has_lowercases(message: &str) -> bool {
    message
        .chars()
        .filter(|c| c.is_lowercase())
        .collect::<Vec<_>>()
        .len()
        > 0
}

fn contains_letters(message: &str) -> bool {
    message
        .chars()
        .filter(|c| c.is_ascii_alphabetic())
        .collect::<Vec<_>>()
        .len()
        > 0
}

fn is_yellowing(message: &str) -> bool {
    contains_letters(message) && !has_lowercases(message)
}

pub fn reply(message: &str) -> &str {
    let trimmed = message.trim();
    if trimmed.len() == 0 {
        return "Fine. Be that way!";
    }
    match (is_question(trimmed), is_yellowing(trimmed)) {
        (true, true) => "Calm down, I know what I'm doing!",
        (true, false) => "Sure.",
        (false, true) => "Whoa, chill out!",
        (_, _) => "Whatever.",
    }
}
