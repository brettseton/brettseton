use crate::apps::AppLine;
use crate::terminal::model::ScreenLine;

#[derive(Clone, Copy, Eq, PartialEq)]
enum ActiveScreen {
    Primary,
    Alternate,
}

#[derive(Default)]
struct ScreenBuffer {
    lines: Vec<ScreenLine>,
}

pub struct TerminalEmulator {
    primary: ScreenBuffer,
    alternate: ScreenBuffer,
    active: ActiveScreen,
}

impl TerminalEmulator {
    pub fn new(lines: Vec<ScreenLine>) -> Self {
        Self {
            primary: ScreenBuffer::new(lines),
            alternate: ScreenBuffer::default(),
            active: ActiveScreen::Primary,
        }
    }

    pub fn reset_primary(&mut self, lines: Vec<ScreenLine>) {
        self.primary = ScreenBuffer::new(lines);
        self.alternate.clear();
        self.active = ActiveScreen::Primary;
    }

    pub fn write_command(&mut self, command: &str) {
        self.primary.push(ScreenLine::command(command.to_string()));
    }

    pub fn write_output_lines(&mut self, lines: Vec<String>) {
        self.primary
            .extend(lines.into_iter().map(ScreenLine::output));
    }

    pub fn sync_alternate_screen(&mut self, lines: &[AppLine], enabled: bool) {
        if enabled {
            self.alternate
                .replace(lines.iter().map(ScreenLine::from_app_line).collect());
            self.active = ActiveScreen::Alternate;
        } else {
            self.alternate.clear();
            self.active = ActiveScreen::Primary;
        }
    }

    pub fn visible_lines(&self) -> &[ScreenLine] {
        match self.active {
            ActiveScreen::Primary => self.primary.lines(),
            ActiveScreen::Alternate => self.alternate.lines(),
        }
    }
}

impl Default for TerminalEmulator {
    fn default() -> Self {
        Self::new(Vec::new())
    }
}

impl ScreenBuffer {
    fn new(lines: Vec<ScreenLine>) -> Self {
        Self { lines }
    }

    fn lines(&self) -> &[ScreenLine] {
        &self.lines
    }

    fn clear(&mut self) {
        self.lines.clear();
    }

    fn push(&mut self, line: ScreenLine) {
        self.lines.push(line);
    }

    fn extend(&mut self, lines: impl IntoIterator<Item = ScreenLine>) {
        self.lines.extend(lines);
    }

    fn replace(&mut self, lines: Vec<ScreenLine>) {
        self.lines = lines;
    }
}
