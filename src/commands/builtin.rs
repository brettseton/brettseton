use crate::apps::AppKind;

use super::{BuiltinCommand, CommandContext, CommandResult};

pub struct ClearCommand;
pub struct GuessCommand;
pub struct HelpCommand;
pub struct LsCommand;
pub struct SnakeCommand;

impl BuiltinCommand for ClearCommand {
    fn name(&self) -> &'static str {
        "clear"
    }

    fn execute(&self, _target: &str, _context: &CommandContext<'_>) -> CommandResult {
        CommandResult::reset_terminal()
    }
}

impl BuiltinCommand for GuessCommand {
    fn name(&self) -> &'static str {
        "guess"
    }

    fn execute(&self, target: &str, _context: &CommandContext<'_>) -> CommandResult {
        launch_app_command(target, "guess", AppKind::Guess)
    }
}

impl BuiltinCommand for HelpCommand {
    fn name(&self) -> &'static str {
        "help"
    }

    fn aliases(&self) -> &'static [&'static str] {
        &["?"]
    }

    fn execute(&self, _target: &str, context: &CommandContext<'_>) -> CommandResult {
        CommandResult::with_lines(
            std::iter::once("commands:".to_string())
                .chain(context.available_commands())
                .collect(),
        )
    }
}

impl BuiltinCommand for LsCommand {
    fn name(&self) -> &'static str {
        "ls"
    }

    fn execute(&self, target: &str, context: &CommandContext<'_>) -> CommandResult {
        match context.resolve_directory(target) {
            Some(entries) => CommandResult::with_lines(vec![entries.join("  ")]),
            None => CommandResult::with_lines(vec![format!(
                "ls: {}: no such directory",
                if target.is_empty() {
                    "missing target"
                } else {
                    target
                }
            )]),
        }
    }
}

impl BuiltinCommand for SnakeCommand {
    fn name(&self) -> &'static str {
        "snake"
    }

    fn execute(&self, target: &str, _context: &CommandContext<'_>) -> CommandResult {
        launch_app_command(target, "snake", AppKind::Snake)
    }
}

fn launch_app_command(target: &str, name: &str, kind: AppKind) -> CommandResult {
    if target.is_empty() {
        CommandResult::with_launch(kind, Vec::new())
    } else {
        CommandResult::with_lines(vec![format!("usage: {name}")])
    }
}
