use crate::commands::{BuiltinCommand, CommandRegistry, CommandResult};

pub struct HelpCommand;

impl BuiltinCommand for HelpCommand {
    fn name(&self) -> &'static str {
        "help"
    }

    fn aliases(&self) -> &'static [&'static str] {
        &["?"]
    }

    fn execute(&self, _target: &str, registry: &CommandRegistry) -> CommandResult {
        CommandResult::with_lines(
            std::iter::once("commands:".to_string())
                .chain(
                    registry
                        .available_commands()
                        .into_iter()
                        .map(str::to_string),
                )
                .collect(),
        )
    }
}
