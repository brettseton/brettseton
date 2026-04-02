use crate::apps::AppKind;
use crate::commands::{BuiltinCommand, CommandRegistry, CommandResult};

use super::launch_app_command;

pub struct SnakeCommand;

impl BuiltinCommand for SnakeCommand {
    fn name(&self) -> &'static str {
        "snake"
    }

    fn execute(&self, target: &str, _registry: &CommandRegistry) -> CommandResult {
        launch_app_command(target, "snake", AppKind::Snake)
    }
}
