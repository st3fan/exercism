fn is_question(message: &str) -> bool {
    message.trim().ends_with("?")
}

fn is_shouted(message: &str) -> bool {
    // TODO Is it possible to do this in one pass? First check is because all() returns true on empty input.
    message.chars().filter(|c| c.is_alphabetic()).count() != 0
        && message
            .chars()
            .filter(|c| c.is_alphabetic())
            .all(|c| c.is_uppercase())
}

fn is_forceful_question(message: &str) -> bool {
    is_shouted(message) && is_question(message)
}

fn is_nothing(message: &str) -> bool {
    message.trim().is_empty()
}

const QUESTION_ANSWER: &str = "Sure.";
const DEFAULT_ANSWER: &str = "Whatever.";
const SHOUTED_ANSWER: &str = "Whoa, chill out!";
const NOTHING_ANSWER: &str = "Fine. Be that way!";
const FORCEFUL_QUESTION_ANSWER: &str = "Calm down, I know what I'm doing!";

pub fn reply(message: &str) -> &str {
    if is_nothing(message) {
        return NOTHING_ANSWER;
    }

    if is_forceful_question(message) {
        return FORCEFUL_QUESTION_ANSWER;
    }

    if is_question(message) {
        return QUESTION_ANSWER;
    }

    if is_shouted(message) {
        return SHOUTED_ANSWER;
    }

    DEFAULT_ANSWER
}
