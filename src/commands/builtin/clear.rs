use crate::commands::{BuiltinCommand, CommandRegistry, CommandResult};

pub struct ClearCommand;

impl BuiltinCommand for ClearCommand {
    fn name(&self) -> &'static str {
        "clear"
    }

    fn execute(&self, _target: &str, _registry: &CommandRegistry) -> CommandResult {
        CommandResult::reset_terminal()
    }
}
