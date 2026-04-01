use js_sys::Math;

use crate::apps::{AppOutput, AppRuntime};

const START_LINES: &[&str] = &[
    "guessing game started",
    "pick a number from 1 to 100",
    "type a number to guess, `new` for a new game, or `quit` to leave",
];
const INVALID_INPUT_LINES: &[&str] = &[
    "enter a number between 1 and 100",
    "or type `new` or `quit`",
];

pub struct GuessGame {
    target: u8,
    attempts: u8,
}

pub fn launch() -> (Box<dyn AppRuntime>, Vec<String>) {
    let game = GuessGame::random();
    (Box::new(game), lines(START_LINES))
}

impl AppRuntime for GuessGame {
    fn handle_command(&mut self, input: &str) -> AppOutput {
        let normalized = input.trim().to_lowercase();

        if normalized == "quit" || normalized == "exit" {
            return AppOutput::Exit(vec!["game over".to_string()]);
        }

        if normalized == "new" {
            let (next_game, lines) = GuessGame::new();
            *self = next_game;
            return AppOutput::Continue(lines);
        }

        let Ok(guess) = normalized.parse::<u8>() else {
            return AppOutput::Continue(lines(INVALID_INPUT_LINES));
        };

        if !(1..=100).contains(&guess) {
            return AppOutput::Continue(vec!["guess must be between 1 and 100".to_string()]);
        }

        self.attempts += 1;

        if guess < self.target {
            return AppOutput::Continue(vec!["too low".to_string()]);
        }

        if guess > self.target {
            return AppOutput::Continue(vec!["too high".to_string()]);
        }

        let attempts = self.attempts;
        AppOutput::Exit(vec![
            format!(
                "correct in {attempts} guess{}",
                if attempts == 1 { "" } else { "es" }
            ),
            "type `guess` to play again".to_string(),
        ])
    }
}

impl GuessGame {
    fn new() -> (Self, Vec<String>) {
        (Self::random(), lines(START_LINES))
    }

    fn random() -> Self {
        Self {
            target: (Math::random() * 100.0).floor() as u8 + 1,
            attempts: 0,
        }
    }
}

fn lines(values: &[&str]) -> Vec<String> {
    values.iter().map(|value| (*value).to_string()).collect()
}
