mod about;
mod clear;
mod github;
mod guess;
mod help;
mod linkedin;
mod snake;
mod stack;
mod tetris;

use crate::apps::AppKind;

use super::CommandResult;

pub use about::AboutCommand;
pub use clear::ClearCommand;
pub use github::GithubCommand;
pub use guess::GuessCommand;
pub use help::HelpCommand;
pub use linkedin::LinkedInCommand;
pub use snake::SnakeCommand;
pub use stack::StackCommand;
pub use tetris::TetrisCommand;

fn launch_app_command(target: &str, name: &str, kind: AppKind) -> CommandResult {
    if target.is_empty() {
        CommandResult::with_program(kind, Vec::new())
    } else {
        CommandResult::with_lines(vec![format!("usage: {name}")])
    }
}

fn launch_link_command(target: &str, name: &str, href: &str) -> CommandResult {
    if target.is_empty() {
        CommandResult::with_open_link(href.to_string(), vec![format!("opening {name}...")])
    } else {
        CommandResult::with_lines(vec![format!("usage: {name}")])
    }
}
