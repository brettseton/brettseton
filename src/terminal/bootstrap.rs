use crate::terminal::model::Entry;

const BANNER_TEXT: &str = "terminal v1.0";
const HELP_PROMPT_TEXT: &str = "type `help` to begin";

pub fn initial_history() -> Vec<Entry> {
    vec![
        Entry::output(BANNER_TEXT.to_string()),
        Entry::output(HELP_PROMPT_TEXT.to_string()),
    ]
}
