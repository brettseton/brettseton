use crate::commands::{BuiltinCommand, CommandRegistry, CommandResult};

const STACK_LINES: &[&str] = &["rust", "webassembly", "html", "css"];

pub struct StackCommand;

impl BuiltinCommand for StackCommand {
    fn name(&self) -> &'static str {
        "stack"
    }

    fn execute(&self, target: &str, _registry: &CommandRegistry) -> CommandResult {
        if target.is_empty() {
            CommandResult::with_lines(STACK_LINES.iter().map(|line| (*line).to_string()).collect())
        } else {
            CommandResult::with_lines(vec!["usage: stack".to_string()])
        }
    }
}
