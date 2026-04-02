use crate::commands::{BuiltinCommand, CommandRegistry, CommandResult};

use super::launch_link_command;

const LINKEDIN_URL: &str = "https://www.linkedin.com/in/brett-seton-b00776195";

pub struct LinkedInCommand;

impl BuiltinCommand for LinkedInCommand {
    fn name(&self) -> &'static str {
        "linkedin"
    }

    fn execute(&self, target: &str, _registry: &CommandRegistry) -> CommandResult {
        launch_link_command(target, "linkedin", LINKEDIN_URL)
    }
}
