use std::collections::BTreeMap;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

const ROOT_DIRECTORY: &str = "src/terminal-fs";
const ROOT_COMMAND_ENTRIES: &[&str] = &["guess", "help", "snake"];

fn main() -> io::Result<()> {
    println!("cargo:rerun-if-changed={ROOT_DIRECTORY}");

    let mut directories: BTreeMap<String, Vec<String>> = BTreeMap::new();
    let mut files: BTreeMap<String, Vec<String>> = BTreeMap::new();
    let mut links: BTreeMap<String, String> = BTreeMap::new();

    walk(
        Path::new(ROOT_DIRECTORY),
        "",
        &mut directories,
        &mut files,
        &mut links,
    )?;

    let mut root_entries = ROOT_COMMAND_ENTRIES
        .iter()
        .map(|item| (*item).to_string())
        .collect::<Vec<_>>();
    root_entries.extend(files.keys().map(|item| basename(item).to_string()));
    root_entries.extend(links.keys().map(|item| basename(item).to_string()));
    root_entries.sort();
    root_entries.dedup();
    directories.insert(".".to_string(), root_entries);

    let out_dir = PathBuf::from(std::env::var("OUT_DIR").expect("OUT_DIR is set by cargo"));
    let destination = out_dir.join("terminal_fs_data.rs");
    let mut output = fs::File::create(destination)?;

    writeln!(
        output,
        "pub const DIRECTORIES: &[(&str, &[&str])] = &{};",
        render_directories(&directories)
    )?;
    writeln!(
        output,
        "pub const FILES: &[(&str, &[&str])] = &{};",
        render_directories(&files)
    )?;
    writeln!(
        output,
        "pub const LINKS: &[(&str, &str)] = &{};",
        render_links(&links)
    )?;

    Ok(())
}

fn walk(
    root: &Path,
    relative_dir: &str,
    directories: &mut BTreeMap<String, Vec<String>>,
    files: &mut BTreeMap<String, Vec<String>>,
    links: &mut BTreeMap<String, String>,
) -> io::Result<()> {
    let absolute_dir = if relative_dir.is_empty() {
        root.to_path_buf()
    } else {
        root.join(relative_dir)
    };

    let mut listing = Vec::new();
    let mut entries = fs::read_dir(&absolute_dir)?.collect::<Result<Vec<_>, _>>()?;
    entries.sort_by_key(|entry| entry.file_name());

    for entry in entries {
        let file_type = entry.file_type()?;
        let name = entry.file_name().to_string_lossy().to_string();
        let relative_path = if relative_dir.is_empty() {
            name.clone()
        } else {
            format!("{relative_dir}/{name}")
        };

        if file_type.is_dir() {
            listing.push(format!("{name}/"));
            walk(root, &relative_path, directories, files, links)?;
            continue;
        }

        listing.push(name);
        let raw = fs::read_to_string(entry.path())?;

        if relative_path.starts_with("links/") {
            links.insert(relative_path, raw.trim().to_string());
        } else {
            files.insert(relative_path, normalize_lines(&raw));
        }
    }

    let key = if relative_dir.is_empty() {
        "."
    } else {
        relative_dir
    };
    directories.insert(key.to_string(), listing);

    Ok(())
}

fn basename(path: &str) -> &str {
    path.rsplit('/').next().unwrap_or(path)
}

fn normalize_lines(raw: &str) -> Vec<String> {
    raw.replace("\r\n", "\n")
        .trim_end()
        .split('\n')
        .map(str::to_string)
        .collect()
}

fn rust_string(value: &str) -> String {
    format!("{value:?}")
}

fn render_map<V, F>(map: &BTreeMap<String, V>, renderer: F) -> String
where
    F: Fn(&V) -> String,
{
    let entries = map
        .iter()
        .map(|(key, value)| format!("({}, {})", rust_string(key), renderer(value)))
        .collect::<Vec<_>>()
        .join(", ");

    format!("[{entries}]")
}

fn render_directories(map: &BTreeMap<String, Vec<String>>) -> String {
    render_map(map, |v| {
        let items = v
            .iter()
            .map(|item| rust_string(item))
            .collect::<Vec<_>>()
            .join(", ");
        format!("&[{items}]")
    })
}

fn render_links(map: &BTreeMap<String, String>) -> String {
    render_map(map, |v| rust_string(v))
}
