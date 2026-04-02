use crate::apps::{AppKey, AppKind, AppLine, AppMode, AppOutput, AppRegistry, AppRuntime};

use super::{State, StateContext, StateTransition, StateUpdate};

pub struct AppState {
    mode: AppMode,
    runtime: Box<dyn AppRuntime>,
    enter_lines: Vec<String>,
    intro_view: Vec<AppLine>,
}

impl AppState {
    pub fn launch(kind: AppKind) -> Self {
        let (runtime, enter_lines) = AppRegistry::launch(kind);
        let mode = runtime.mode();
        let intro_view = if mode == AppMode::Realtime {
            enter_lines.iter().cloned().map(AppLine::text).collect()
        } else {
            Vec::new()
        };
        Self {
            mode,
            runtime,
            enter_lines,
            intro_view,
        }
    }

    fn from_output(output: AppOutput) -> StateTransition {
        match output {
            AppOutput::Continue(lines) => {
                let update = if lines.is_empty() {
                    StateUpdate::for_view_refresh()
                } else {
                    StateUpdate {
                        refresh_view: true,
                        ..StateUpdate::with_lines(lines)
                    }
                };
                StateTransition::Stay(update)
            }
            AppOutput::Exit(lines) => StateTransition::Exit(StateUpdate {
                refresh_view: true,
                ..StateUpdate::with_lines(lines)
            }),
        }
    }
}

impl State for AppState {
    fn on_enter(&mut self) -> Vec<String> {
        self.runtime.on_enter();
        if self.mode == AppMode::Realtime {
            Vec::new()
        } else {
            std::mem::take(&mut self.enter_lines)
        }
    }

    fn on_exit(&mut self) {
        self.runtime.on_exit();
    }

    fn handle_command(&mut self, input: &str, _context: &StateContext<'_>) -> StateTransition {
        if self.mode == AppMode::Realtime {
            return StateTransition::stay();
        }

        Self::from_output(self.runtime.handle_command(input))
    }

    fn handle_key(&mut self, key: AppKey) -> StateTransition {
        self.runtime
            .handle_key(key)
            .map(Self::from_output)
            .unwrap_or_else(StateTransition::stay)
    }

    fn tick(&mut self) -> StateTransition {
        if self.mode != AppMode::Realtime {
            return StateTransition::stay();
        }

        self.runtime
            .tick()
            .map(Self::from_output)
            .unwrap_or_else(StateTransition::stay)
    }

    fn view(&self) -> Vec<AppLine> {
        if self.mode != AppMode::Realtime {
            return self.runtime.view();
        }

        let mut lines = self.intro_view.clone();
        lines.extend(self.runtime.view());
        lines
    }

    fn prompt_enabled(&self) -> bool {
        self.mode == AppMode::Interactive
    }

    fn captures_keyboard(&self) -> bool {
        self.mode == AppMode::Realtime
    }

    fn uses_alternate_screen(&self) -> bool {
        self.mode == AppMode::Realtime
    }
}
