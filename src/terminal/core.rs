use std::collections::BTreeSet;

use crate::apps::{AppKey, AppKind, AppOutput, AppRuntime};
use crate::commands::{
    Executable, TerminalActions, aliases as command_aliases, names as command_names,
    resolve as resolve_command,
};
use crate::terminal_fs::TerminalFs;

const BANNER_TEXT: &str = "terminal v1.0";
const HELP_PROMPT_TEXT: &str = "type `help` to begin";

#[derive(Clone)]
pub struct Entry {
    pub kind: EntryKind,
    pub text: String,
}

#[derive(Clone, Copy)]
pub enum EntryKind {
    Command,
    Output,
}

impl EntryKind {
    pub fn prefix(self) -> &'static str {
        match self {
            Self::Command => "$",
            Self::Output => ">",
        }
    }

    pub fn class_name(self) -> &'static str {
        match self {
            Self::Command => "line-prefix command",
            Self::Output => "line-prefix output",
        }
    }
}

pub enum Effect {
    OpenLink(String),
}

pub struct TerminalCore {
    session: TerminalSession,
    app_host: AppHost,
}

impl TerminalCore {
    pub fn new(terminal_fs: TerminalFs) -> Self {
        Self {
            session: TerminalSession::new(terminal_fs),
            app_host: AppHost::new(),
        }
    }

    pub fn history(&self) -> &[Entry] {
        self.session.history()
    }

    pub fn active_view(&self) -> &[String] {
        self.app_host.active_view()
    }

    pub fn captures_keyboard(&self) -> bool {
        self.app_host.is_realtime()
    }

    pub fn links(&self) -> &std::collections::BTreeMap<String, String> {
        self.session.links()
    }

    pub fn run_command(&mut self, raw: String) -> Vec<Effect> {
        let trimmed = raw.trim().to_string();

        if self.app_host.is_active() {
            if self.app_host.is_realtime() {
                return Vec::new();
            }

            self.session.record_command(&trimmed);

            if trimmed.is_empty() {
                return Vec::new();
            }

            self.handle_app_input(&trimmed);
            return Vec::new();
        }

        self.session.record_command(&trimmed);

        if trimmed.is_empty() {
            return Vec::new();
        }

        let mut parts = trimmed.split_whitespace();
        let command = parts.next().unwrap_or_default().to_string();
        let target = parts.collect::<Vec<_>>().join(" ");

        match resolve_command(&command, self.session.terminal_fs()) {
            Some(Executable::Builtin(command)) => {
                command.execute(&target, self);
            }
            Some(Executable::External(item)) if target.is_empty() => {
                item.execute(&command, self);
            }
            Some(Executable::External(_)) if !target.is_empty() => {
                self.session
                    .append_output(vec![format!("{command}: command not found")]);
            }
            Some(Executable::External(_)) => {
                self.session.append_output(vec![format!("{command}: not found")]);
            }
            None if !target.is_empty() => {
                self.session
                    .append_output(vec![format!("{command}: command not found")]);
            }
            None => {
                self.session.append_output(vec![format!("{command}: not found")]);
            }
        }

        self.session.take_effects()
    }

    pub fn show_help(&mut self) {
        let mut items: BTreeSet<String> = self.session.root_items().cloned().collect();
        items.extend(command_names().map(str::to_string));
        items.extend(command_aliases().map(str::to_string));

        self.session.append_output(
            std::iter::once("commands:".to_string())
                .chain(items)
                .collect(),
        );
    }

    pub fn list_directory(&mut self, target: &str) {
        match self.session.resolve_directory(target) {
            Some(entries) => self.session.append_output(vec![entries.join("  ")]),
            None => self.session.append_output(vec![format!(
                "ls: {}: no such directory",
                if target.is_empty() {
                    "missing target"
                } else {
                    target
                }
            )]),
        }
    }

    pub fn launch_app(&mut self, kind: AppKind) {
        let lines = self.app_host.launch(kind);
        self.session.append_output(lines);
    }

    pub fn reset_terminal(&mut self) {
        self.app_host.reset();
        self.session.reset();
    }

    pub fn append_output(&mut self, lines: Vec<String>) {
        self.session.append_output(lines);
    }

    pub fn open_link(&mut self, href: String) {
        self.session.open_link(href);
    }

    pub fn handle_key(&mut self, key: AppKey) -> bool {
        let Some(output) = self.app_host.handle_key(key) else {
            return false;
        };

        self.apply_app_output(output);
        true
    }

    pub fn tick(&mut self) -> bool {
        let Some(output) = self.app_host.tick() else {
            return false;
        };

        self.apply_app_output(output);
        true
    }

    fn handle_app_input(&mut self, input: &str) {
        let Some(output) = self.app_host.handle_command(input) else {
            return;
        };
        self.apply_app_output(output);
    }

    fn apply_app_output(&mut self, output: AppOutput) {
        match output {
            AppOutput::Continue(lines) => {
                if !lines.is_empty() {
                    self.session.append_output(lines);
                }
                self.app_host.refresh_view();
            }
            AppOutput::Exit(lines) => {
                self.app_host.reset();
                self.session.append_output(lines);
            }
        }
    }
}

impl TerminalActions for TerminalCore {
    fn show_help(&mut self) {
        Self::show_help(self);
    }

    fn list_directory(&mut self, target: &str) {
        Self::list_directory(self, target);
    }

    fn launch_app(&mut self, kind: AppKind) {
        Self::launch_app(self, kind);
    }

    fn reset_terminal(&mut self) {
        Self::reset_terminal(self);
    }

    fn append_output(&mut self, lines: Vec<String>) {
        Self::append_output(self, lines);
    }

    fn open_link(&mut self, href: String) {
        Self::open_link(self, href);
    }
}

impl Entry {
    fn command(text: String) -> Self {
        Self {
            kind: EntryKind::Command,
            text,
        }
    }

    fn output(text: String) -> Self {
        Self {
            kind: EntryKind::Output,
            text,
        }
    }
}

fn initial_history() -> Vec<Entry> {
    vec![
        Entry::output(BANNER_TEXT.to_string()),
        Entry::output(HELP_PROMPT_TEXT.to_string()),
    ]
}

struct TerminalSession {
    terminal_fs: TerminalFs,
    history: Vec<Entry>,
    effects: Vec<Effect>,
}

impl TerminalSession {
    fn new(terminal_fs: TerminalFs) -> Self {
        Self {
            terminal_fs,
            history: initial_history(),
            effects: Vec::new(),
        }
    }

    fn history(&self) -> &[Entry] {
        &self.history
    }

    fn links(&self) -> &std::collections::BTreeMap<String, String> {
        &self.terminal_fs.links
    }

    fn terminal_fs(&self) -> &TerminalFs {
        &self.terminal_fs
    }

    fn root_items(&self) -> impl Iterator<Item = &String> {
        self.terminal_fs.root_items()
    }

    fn resolve_directory(&self, target: &str) -> Option<&Vec<String>> {
        self.terminal_fs.resolve_directory(target)
    }

    fn reset(&mut self) {
        self.history = initial_history();
        self.effects.clear();
    }

    fn record_command(&mut self, trimmed: &str) {
        self.history.push(Entry::command(if trimmed.is_empty() {
            " ".to_string()
        } else {
            trimmed.to_string()
        }));
    }

    fn append_output(&mut self, lines: Vec<String>) {
        self.history.extend(lines.into_iter().map(Entry::output));
    }

    fn open_link(&mut self, href: String) {
        self.effects.push(Effect::OpenLink(href));
    }

    fn take_effects(&mut self) -> Vec<Effect> {
        std::mem::take(&mut self.effects)
    }
}

struct AppHost {
    active_app: Option<Box<dyn AppRuntime>>,
    active_view: Vec<String>,
}

impl AppHost {
    fn new() -> Self {
        Self {
            active_app: None,
            active_view: Vec::new(),
        }
    }

    fn is_active(&self) -> bool {
        self.active_app.is_some()
    }

    fn is_realtime(&self) -> bool {
        self.active_app
            .as_ref()
            .map(|app| app.is_realtime())
            .unwrap_or(false)
    }

    fn active_view(&self) -> &[String] {
        &self.active_view
    }

    fn launch(&mut self, kind: AppKind) -> Vec<String> {
        let (app, lines) = kind.launch();
        self.active_app = Some(app);
        self.refresh_view();
        lines
    }

    fn reset(&mut self) {
        self.active_app = None;
        self.active_view.clear();
    }

    fn handle_command(&mut self, input: &str) -> Option<AppOutput> {
        let mut app = self.active_app.take()?;
        let output = app.handle_command(input);
        self.active_app = Some(app);
        Some(output)
    }

    fn handle_key(&mut self, key: AppKey) -> Option<AppOutput> {
        let mut app = self.active_app.take()?;
        let output = app.handle_key(key);
        self.active_app = Some(app);
        output
    }

    fn tick(&mut self) -> Option<AppOutput> {
        let mut app = self.active_app.take()?;

        if !app.is_realtime() {
            self.active_app = Some(app);
            return None;
        }

        let output = app.tick();
        self.active_app = Some(app);
        output
    }

    fn refresh_view(&mut self) {
        self.active_view = self
            .active_app
            .as_ref()
            .map(|app| app.view())
            .unwrap_or_default();
    }
}
