use super::BuiltinCommand;
use super::builtin::{ClearCommand, GuessCommand, HelpCommand, LsCommand, SnakeCommand};

static CLEAR_COMMAND: ClearCommand = ClearCommand;
static GUESS_COMMAND: GuessCommand = GuessCommand;
static HELP_COMMAND: HelpCommand = HelpCommand;
static LS_COMMAND: LsCommand = LsCommand;
static SNAKE_COMMAND: SnakeCommand = SnakeCommand;

static COMMANDS: [&dyn BuiltinCommand; 5] = [
    &CLEAR_COMMAND,
    &GUESS_COMMAND,
    &HELP_COMMAND,
    &LS_COMMAND,
    &SNAKE_COMMAND,
];

pub fn find(value: &str) -> Option<&'static dyn BuiltinCommand> {
    COMMANDS
        .iter()
        .copied()
        .find(|command| matches_command(*command, value))
}

pub fn names() -> impl Iterator<Item = &'static str> {
    COMMANDS.iter().map(|command| command.name())
}

pub fn aliases() -> impl Iterator<Item = &'static str> {
    COMMANDS
        .iter()
        .flat_map(|command| command.aliases().iter().copied())
}

fn matches_command(command: &dyn BuiltinCommand, value: &str) -> bool {
    command.name().eq_ignore_ascii_case(value)
        || command
            .aliases()
            .iter()
            .any(|alias| alias.eq_ignore_ascii_case(value))
}
