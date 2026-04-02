mod builtin;
mod registry;

use std::collections::BTreeSet;

use crate::apps::AppKind;
use crate::terminal_fs::FileSystem;

pub struct CommandContext<'a> {
    terminal_fs: &'a dyn FileSystem,
    command_names: &'a [&'static str],
    aliases: &'a [&'static str],
}

impl<'a> CommandContext<'a> {
    pub fn new(
        terminal_fs: &'a dyn FileSystem,
        command_names: &'a [&'static str],
        aliases: &'a [&'static str],
    ) -> Self {
        Self {
            terminal_fs,
            command_names,
            aliases,
        }
    }

    pub fn available_commands(&self) -> BTreeSet<String> {
        let mut items: BTreeSet<String> = self.terminal_fs.root_items().into_iter().collect();
        items.extend(self.command_names.iter().map(|name| (*name).to_string()));
        items.extend(self.aliases.iter().map(|alias| (*alias).to_string()));
        items
    }

    pub fn resolve_directory(&self, target: &str) -> Option<&[String]> {
        self.terminal_fs.resolve_directory(target)
    }
}

#[derive(Default)]
pub struct CommandResult {
    lines: Vec<String>,
    launch_app: Option<AppKind>,
    open_link: Option<String>,
    reset_terminal: bool,
}

impl CommandResult {
    pub fn with_lines(lines: Vec<String>) -> Self {
        Self {
            lines,
            ..Self::default()
        }
    }

    pub fn with_launch(kind: AppKind, lines: Vec<String>) -> Self {
        Self {
            lines,
            launch_app: Some(kind),
            ..Self::default()
        }
    }

    pub fn with_open_link(href: String, lines: Vec<String>) -> Self {
        Self {
            lines,
            open_link: Some(href),
            ..Self::default()
        }
    }

    pub fn reset_terminal() -> Self {
        Self {
            reset_terminal: true,
            ..Self::default()
        }
    }

    pub fn lines(&self) -> &[String] {
        &self.lines
    }

    pub fn launch_app(&self) -> Option<AppKind> {
        self.launch_app
    }

    pub fn open_link(&self) -> Option<&str> {
        self.open_link.as_deref()
    }

    pub fn should_reset_terminal(&self) -> bool {
        self.reset_terminal
    }
}

pub trait BuiltinCommand: Sync {
    fn name(&self) -> &'static str;
    fn aliases(&self) -> &'static [&'static str] {
        &[]
    }
    fn execute(&self, target: &str, context: &CommandContext<'_>) -> CommandResult;
}

pub enum Executable {
    Builtin(&'static dyn BuiltinCommand),
    External(crate::terminal_fs::OwnedTerminalItem),
}

pub fn resolve(value: &str, terminal_fs: &dyn FileSystem) -> Option<Executable> {
    if let Some(command) = registry::find(value) {
        return Some(Executable::Builtin(command));
    }

    terminal_fs
        .resolve_owned_item(value)
        .map(Executable::External)
}

pub fn names() -> impl Iterator<Item = &'static str> {
    registry::names()
}

pub fn aliases() -> impl Iterator<Item = &'static str> {
    registry::aliases()
}
