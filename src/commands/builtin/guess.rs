use crate::apps::AppKind;
use crate::commands::{BuiltinCommand, CommandRegistry, CommandResult};

use super::launch_app_command;

pub struct GuessCommand;

impl BuiltinCommand for GuessCommand {
    fn name(&self) -> &'static str {
        "guess"
    }

    fn execute(&self, target: &str, _registry: &CommandRegistry) -> CommandResult {
        launch_app_command(target, "guess", AppKind::Guess)
    }
}
