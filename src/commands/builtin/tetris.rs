use crate::apps::AppKind;
use crate::commands::{BuiltinCommand, CommandRegistry, CommandResult};

use super::launch_app_command;

pub struct TetrisCommand;

impl BuiltinCommand for TetrisCommand {
    fn name(&self) -> &'static str {
        "tetris"
    }

    fn execute(&self, target: &str, _registry: &CommandRegistry) -> CommandResult {
        launch_app_command(target, "tetris", AppKind::Tetris)
    }
}
