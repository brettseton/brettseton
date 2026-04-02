use crate::commands::{BuiltinCommand, CommandRegistry, CommandResult};

const ABOUT_LINES: &[&str] = &[
    "Brett Seton",
    "Sydney, Australia",
    "Builds software across web, systems, and infrastructure.",
];

pub struct AboutCommand;

impl BuiltinCommand for AboutCommand {
    fn name(&self) -> &'static str {
        "about"
    }

    fn execute(&self, target: &str, _registry: &CommandRegistry) -> CommandResult {
        if target.is_empty() {
            CommandResult::with_lines(ABOUT_LINES.iter().map(|line| (*line).to_string()).collect())
        } else {
            CommandResult::with_lines(vec!["usage: about".to_string()])
        }
    }
}
