mod builtin;
mod registry;

use crate::apps::AppKind;

pub use registry::CommandRegistry;

pub struct CommandInvocation<'a> {
    name: &'a str,
    target: &'a str,
}

#[derive(Default)]
pub struct CommandResult {
    lines: Vec<String>,
    start_program: Option<AppKind>,
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

    pub fn with_program(kind: AppKind, lines: Vec<String>) -> Self {
        Self {
            lines,
            start_program: Some(kind),
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

    pub fn start_program(&self) -> Option<AppKind> {
        self.start_program
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
    fn execute(&self, target: &str, registry: &CommandRegistry) -> CommandResult;
}

fn parse(input: &str) -> CommandInvocation<'_> {
    let mut parts = input.split_whitespace();
    let name = parts.next().unwrap_or_default();
    let target = input[name.len()..].trim_start();
    CommandInvocation { name, target }
}
