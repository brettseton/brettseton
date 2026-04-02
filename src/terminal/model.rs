use crate::apps::AppLine;

#[derive(Clone, Eq, PartialEq)]
pub struct Entry {
    pub kind: EntryKind,
    pub text: String,
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum EntryKind {
    Command,
    Output,
}

pub enum Effect {
    OpenLink(String),
}

pub struct ViewModel {
    history: Vec<Entry>,
    active_view: Vec<AppLine>,
    prompt_enabled: bool,
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

impl ViewModel {
    pub fn new(history: Vec<Entry>, active_view: Vec<AppLine>, prompt_enabled: bool) -> Self {
        Self {
            history,
            active_view,
            prompt_enabled,
        }
    }

    pub fn history(&self) -> &[Entry] {
        &self.history
    }

    pub fn active_view(&self) -> &[AppLine] {
        &self.active_view
    }

    pub fn prompt_enabled(&self) -> bool {
        self.prompt_enabled
    }
}

impl Entry {
    pub(crate) fn command(text: String) -> Self {
        Self {
            kind: EntryKind::Command,
            text,
        }
    }

    pub(crate) fn output(text: String) -> Self {
        Self {
            kind: EntryKind::Output,
            text,
        }
    }
}
