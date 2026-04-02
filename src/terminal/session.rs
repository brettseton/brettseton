use std::collections::BTreeMap;

use crate::terminal::model::{Effect, Entry};
use crate::terminal_fs::FileSystem;

pub struct TerminalSession {
    terminal_fs: Box<dyn FileSystem>,
    history: Vec<Entry>,
    effects: Vec<Effect>,
}

impl TerminalSession {
    pub fn new(terminal_fs: Box<dyn FileSystem>, history: Vec<Entry>) -> Self {
        Self {
            terminal_fs,
            history,
            effects: Vec::new(),
        }
    }

    pub fn history(&self) -> &[Entry] {
        &self.history
    }

    pub fn links(&self) -> &BTreeMap<String, String> {
        self.terminal_fs.links()
    }

    pub fn terminal_fs(&self) -> &dyn FileSystem {
        self.terminal_fs.as_ref()
    }

    pub fn reset(&mut self, history: Vec<Entry>) {
        self.history = history;
        self.effects.clear();
    }

    pub fn record_command(&mut self, trimmed: &str) {
        self.history.push(Entry::command(if trimmed.is_empty() {
            " ".to_string()
        } else {
            trimmed.to_string()
        }));
    }

    pub fn append_output(&mut self, lines: Vec<String>) {
        self.history.extend(lines.into_iter().map(Entry::output));
    }

    pub fn push_effect(&mut self, effect: Effect) {
        self.effects.push(effect);
    }

    pub fn take_effects(&mut self) -> Vec<Effect> {
        std::mem::take(&mut self.effects)
    }
}
