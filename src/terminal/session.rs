use crate::apps::AppLine;
use crate::terminal::emulator::TerminalEmulator;
use crate::terminal::model::{Effect, ScreenLine, ViewModel};

pub struct TerminalSession {
    emulator: TerminalEmulator,
    effects: Vec<Effect>,
}

impl TerminalSession {
    pub fn new(initial_screen: Vec<ScreenLine>) -> Self {
        Self {
            emulator: TerminalEmulator::new(initial_screen),
            effects: Vec::new(),
        }
    }

    pub fn reset(&mut self, initial_screen: Vec<ScreenLine>) {
        self.emulator.reset_primary(initial_screen);
        self.effects.clear();
    }

    pub fn record_command(&mut self, trimmed: &str) {
        self.emulator
            .write_command(if trimmed.is_empty() { " " } else { trimmed });
    }

    pub fn append_output(&mut self, lines: Vec<String>) {
        self.emulator.write_output_lines(lines);
    }

    pub fn sync_state_view(&mut self, lines: Vec<AppLine>, use_alternate_screen: bool) {
        self.emulator
            .sync_alternate_screen(&lines, use_alternate_screen);
    }

    pub fn push_effect(&mut self, effect: Effect) {
        self.effects.push(effect);
    }

    pub fn take_effects(&mut self) -> Vec<Effect> {
        std::mem::take(&mut self.effects)
    }

    pub fn view(&self, prompt_enabled: bool) -> ViewModel {
        ViewModel::new(self.emulator.visible_lines().to_vec(), prompt_enabled)
    }
}
