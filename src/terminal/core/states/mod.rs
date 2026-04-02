mod app_state;
mod shell_state;

use crate::apps::{AppKey, AppLine};
use crate::commands::CommandRegistry;
use crate::terminal::model::Effect;

pub use app_state::AppState;
pub use shell_state::ShellState;

pub trait State {
    fn on_enter(&mut self) -> Vec<String> {
        Vec::new()
    }

    fn on_exit(&mut self) {}

    fn handle_command(&mut self, input: &str, context: &StateContext<'_>) -> StateTransition;

    fn handle_key(&mut self, _key: AppKey) -> StateTransition {
        StateTransition::stay()
    }

    fn tick(&mut self) -> StateTransition {
        StateTransition::stay()
    }

    fn view(&self) -> Vec<AppLine> {
        Vec::new()
    }

    fn prompt_enabled(&self) -> bool {
        true
    }

    fn captures_keyboard(&self) -> bool {
        false
    }

    fn uses_alternate_screen(&self) -> bool {
        false
    }
}

pub struct StateContext<'a> {
    pub command_registry: &'a CommandRegistry,
}

#[derive(Default)]
pub struct StateUpdate {
    pub lines: Vec<String>,
    pub effects: Vec<Effect>,
    pub reset_terminal: bool,
    pub refresh_view: bool,
}

pub enum StateTransition {
    Stay(StateUpdate),
    Change {
        next: Box<dyn State>,
        update: StateUpdate,
    },
    Exit(StateUpdate),
}

impl StateContext<'_> {
    pub fn new<'a>(command_registry: &'a CommandRegistry) -> StateContext<'a> {
        StateContext { command_registry }
    }
}

impl StateUpdate {
    pub fn with_lines(lines: Vec<String>) -> Self {
        Self {
            lines,
            ..Self::default()
        }
    }

    pub fn for_view_refresh() -> Self {
        Self {
            refresh_view: true,
            ..Self::default()
        }
    }

    pub fn changes_view(&self) -> bool {
        self.refresh_view || self.reset_terminal || !self.lines.is_empty()
    }
}

impl StateTransition {
    pub fn stay() -> Self {
        Self::Stay(StateUpdate::default())
    }
}
