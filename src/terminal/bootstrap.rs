use crate::terminal::model::ScreenLine;

const BANNER_TEXT: &str = "terminal v1.0";
const HELP_PROMPT_TEXT: &str = "type `help` to begin";

pub fn initial_screen() -> Vec<ScreenLine> {
    vec![
        ScreenLine::output(BANNER_TEXT.to_string()),
        ScreenLine::output(HELP_PROMPT_TEXT.to_string()),
    ]
}
