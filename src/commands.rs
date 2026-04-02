use crate::apps::AppKind;

pub trait TerminalActions {
    fn show_help(&mut self);
    fn list_directory(&mut self, target: &str);
    fn launch_app(&mut self, kind: AppKind);
    fn reset_terminal(&mut self);
    fn append_output(&mut self, lines: Vec<String>);
    fn open_link(&mut self, href: String);
}

pub trait BuiltinCommand: Sync {
    fn name(&self) -> &'static str;
    fn aliases(&self) -> &'static [&'static str] {
        &[]
    }
    fn execute(&self, target: &str, terminal: &mut dyn TerminalActions);
}

struct ClearCommand;
struct GameCommand;
struct HelpCommand;
struct LsCommand;
struct SnakeCommand;

static CLEAR_COMMAND: ClearCommand = ClearCommand;
static GAME_COMMAND: GameCommand = GameCommand;
static HELP_COMMAND: HelpCommand = HelpCommand;
static LS_COMMAND: LsCommand = LsCommand;
static SNAKE_COMMAND: SnakeCommand = SnakeCommand;

static COMMANDS: [&'static dyn BuiltinCommand; 5] = [
    &CLEAR_COMMAND,
    &GAME_COMMAND,
    &HELP_COMMAND,
    &LS_COMMAND,
    &SNAKE_COMMAND,
];

pub enum Executable {
    Builtin(&'static dyn BuiltinCommand),
    External(crate::terminal_fs::OwnedTerminalItem),
}

pub fn find(value: &str) -> Option<&'static dyn BuiltinCommand> {
    COMMANDS
        .iter()
        .copied()
        .find(|command| matches_command(*command, value))
}

pub fn resolve(value: &str, terminal_fs: &crate::terminal_fs::TerminalFs) -> Option<Executable> {
    if let Some(command) = find(value) {
        return Some(Executable::Builtin(command));
    }

    terminal_fs
        .resolve_owned_item(value)
        .map(Executable::External)
}

pub fn names() -> impl Iterator<Item = &'static str> {
    COMMANDS.iter().map(|command| command.name())
}

pub fn aliases() -> impl Iterator<Item = &'static str> {
    COMMANDS
        .iter()
        .flat_map(|command| command.aliases().iter().copied())
}

impl BuiltinCommand for ClearCommand {
    fn name(&self) -> &'static str {
        "clear"
    }

    fn execute(&self, _target: &str, terminal: &mut dyn TerminalActions) {
        terminal.reset_terminal();
    }
}

impl BuiltinCommand for GameCommand {
    fn name(&self) -> &'static str {
        "guess"
    }

    fn execute(&self, target: &str, terminal: &mut dyn TerminalActions) {
        if target.is_empty() {
            terminal.launch_app(AppKind::Guess);
        } else {
            terminal.append_output(vec!["usage: guess".to_string()]);
        }
    }
}

impl BuiltinCommand for HelpCommand {
    fn name(&self) -> &'static str {
        "help"
    }

    fn aliases(&self) -> &'static [&'static str] {
        &["?"]
    }

    fn execute(&self, _target: &str, terminal: &mut dyn TerminalActions) {
        terminal.show_help();
    }
}

impl BuiltinCommand for LsCommand {
    fn name(&self) -> &'static str {
        "ls"
    }

    fn execute(&self, target: &str, terminal: &mut dyn TerminalActions) {
        terminal.list_directory(target);
    }
}

impl BuiltinCommand for SnakeCommand {
    fn name(&self) -> &'static str {
        "snake"
    }

    fn execute(&self, target: &str, terminal: &mut dyn TerminalActions) {
        if target.is_empty() {
            terminal.launch_app(AppKind::Snake);
        } else {
            terminal.append_output(vec!["usage: snake".to_string()]);
        }
    }
}

fn matches_command(command: &dyn BuiltinCommand, value: &str) -> bool {
    command.name().eq_ignore_ascii_case(value)
        || command
            .aliases()
            .iter()
            .any(|alias| alias.eq_ignore_ascii_case(value))
}
