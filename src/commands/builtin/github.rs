use crate::commands::{BuiltinCommand, CommandRegistry, CommandResult};

use super::launch_link_command;

const GITHUB_URL: &str = "https://github.com/brettseton";

pub struct GithubCommand;

impl BuiltinCommand for GithubCommand {
    fn name(&self) -> &'static str {
        "github"
    }

    fn execute(&self, target: &str, _registry: &CommandRegistry) -> CommandResult {
        launch_link_command(target, "github", GITHUB_URL)
    }
}
