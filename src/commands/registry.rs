use std::collections::BTreeSet;

use super::builtin::{
    AboutCommand, ClearCommand, GithubCommand, GuessCommand, HelpCommand, LinkedInCommand,
    SnakeCommand, StackCommand,
};
use super::{BuiltinCommand, CommandResult, parse};

static ABOUT_COMMAND: AboutCommand = AboutCommand;
static CLEAR_COMMAND: ClearCommand = ClearCommand;
static GITHUB_COMMAND: GithubCommand = GithubCommand;
static GUESS_COMMAND: GuessCommand = GuessCommand;
static HELP_COMMAND: HelpCommand = HelpCommand;
static LINKEDIN_COMMAND: LinkedInCommand = LinkedInCommand;
static SNAKE_COMMAND: SnakeCommand = SnakeCommand;
static STACK_COMMAND: StackCommand = StackCommand;

static COMMANDS: [&dyn BuiltinCommand; 8] = [
    &ABOUT_COMMAND,
    &CLEAR_COMMAND,
    &GITHUB_COMMAND,
    &GUESS_COMMAND,
    &HELP_COMMAND,
    &LINKEDIN_COMMAND,
    &SNAKE_COMMAND,
    &STACK_COMMAND,
];

#[derive(Clone, Copy, Default)]
pub struct CommandRegistry;

impl CommandRegistry {
    pub fn new() -> Self {
        Self
    }

    pub fn execute(&self, input: &str) -> CommandResult {
        let invocation = parse(input);

        match self.find(invocation.name) {
            Some(command) => command.execute(invocation.target, self),
            None => CommandResult::with_lines(vec![if invocation.target.is_empty() {
                format!("{}: not found", invocation.name)
            } else {
                format!("{}: command not found", invocation.name)
            }]),
        }
    }

    pub fn find(&self, value: &str) -> Option<&'static dyn BuiltinCommand> {
        COMMANDS
            .iter()
            .copied()
            .find(|command| matches_command(*command, value))
    }

    pub fn names(&self) -> impl Iterator<Item = &'static str> {
        COMMANDS.iter().map(|command| command.name())
    }

    pub fn aliases(&self) -> impl Iterator<Item = &'static str> {
        COMMANDS
            .iter()
            .flat_map(|command| command.aliases().iter().copied())
    }

    pub fn available_commands(&self) -> BTreeSet<&'static str> {
        let mut items = BTreeSet::new();
        items.extend(self.names());
        items.extend(self.aliases());
        items
    }
}

fn matches_command(command: &dyn BuiltinCommand, value: &str) -> bool {
    command.name().eq_ignore_ascii_case(value)
        || command
            .aliases()
            .iter()
            .any(|alias| alias.eq_ignore_ascii_case(value))
}
