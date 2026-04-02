use std::collections::BTreeMap;

include!(concat!(env!("OUT_DIR"), "/terminal_fs_data.rs"));

use crate::commands::CommandResult;

pub enum TerminalItem<'a> {
    File(&'a [String]),
    Link(&'a str),
}

pub enum OwnedTerminalItem {
    File(Vec<String>),
    Link(String),
}

impl OwnedTerminalItem {
    pub fn execute(&self, target: &str) -> CommandResult {
        match self {
            Self::File(file) => CommandResult::with_lines(file.clone()),
            Self::Link(link) => CommandResult::with_open_link(
                link.clone(),
                vec![format!("opening {}...", basename(target))],
            ),
        }
    }
}

pub trait FileSystem {
    fn resolve_directory(&self, value: &str) -> Option<&[String]>;
    fn root_items(&self) -> Vec<String>;
    fn resolve_owned_item(&self, value: &str) -> Option<OwnedTerminalItem>;
    fn links(&self) -> &BTreeMap<String, String>;
}

#[derive(Clone)]
pub struct TerminalFs {
    pub directories: BTreeMap<String, Vec<String>>,
    pub files: BTreeMap<String, Vec<String>>,
    pub links: BTreeMap<String, String>,
}

pub fn load_terminal_fs() -> TerminalFs {
    TerminalFs {
        directories: load_array_map(DIRECTORIES),
        files: load_array_map(FILES),
        links: load_string_map(LINKS),
    }
}

impl TerminalFs {
    pub fn resolve_directory(&self, value: &str) -> Option<&[String]> {
        let target = normalize_target(value);
        self.directories.get(&target).map(Vec::as_slice)
    }

    pub fn root_items(&self) -> impl Iterator<Item = &String> {
        self.directories.get(".").into_iter().flatten()
    }

    pub fn resolve_item(&self, value: &str) -> Option<TerminalItem<'_>> {
        let resolved = self.resolve_item_target(value)?;

        if let Some(file) = self.files.get(&resolved) {
            return Some(TerminalItem::File(file));
        }

        self.links
            .get(&resolved)
            .map(|link| TerminalItem::Link(link.as_str()))
    }

    pub fn resolve_owned_item(&self, value: &str) -> Option<OwnedTerminalItem> {
        self.resolve_item(value).map(|item| match item {
            TerminalItem::File(file) => OwnedTerminalItem::File(file.to_vec()),
            TerminalItem::Link(link) => OwnedTerminalItem::Link(link.to_string()),
        })
    }

    fn resolve_item_target(&self, value: &str) -> Option<String> {
        let target = normalize_target(value);

        if self.files.contains_key(&target) || self.links.contains_key(&target) {
            return Some(target);
        }

        let mut basename_matches = self
            .files
            .keys()
            .chain(self.links.keys())
            .filter(|item| basename(item) == target)
            .map(String::as_str);

        let first = basename_matches.next()?;
        if basename_matches.next().is_some() {
            return None;
        }

        Some(first.to_string())
    }
}

impl FileSystem for TerminalFs {
    fn resolve_directory(&self, value: &str) -> Option<&[String]> {
        Self::resolve_directory(self, value)
    }

    fn root_items(&self) -> Vec<String> {
        Self::root_items(self).cloned().collect()
    }

    fn resolve_owned_item(&self, value: &str) -> Option<OwnedTerminalItem> {
        Self::resolve_owned_item(self, value)
    }

    fn links(&self) -> &BTreeMap<String, String> {
        &self.links
    }
}

fn load_array_map(values: &[(&str, &[&str])]) -> BTreeMap<String, Vec<String>> {
    values
        .iter()
        .map(|(k, v)| (k.to_string(), v.iter().map(|s| s.to_string()).collect()))
        .collect()
}

fn load_string_map(values: &[(&str, &str)]) -> BTreeMap<String, String> {
    values
        .iter()
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect()
}

fn normalize_target(value: &str) -> String {
    let target = value
        .trim()
        .strip_prefix("./")
        .unwrap_or(value.trim())
        .trim_matches('/')
        .to_lowercase();

    if target.is_empty() {
        ".".to_string()
    } else {
        target
    }
}

fn basename(path: &str) -> &str {
    path.rsplit('/').next().unwrap_or(path)
}
