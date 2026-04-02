use crate::terminal::bootstrap;
use crate::terminal::core::states::{ShellState, State, StateTransition, StateUpdate};
use crate::terminal::session::TerminalSession;

pub struct StateMachine {
    state: Box<dyn State>,
}

impl StateMachine {
    pub fn new() -> Self {
        Self {
            state: Box::new(ShellState),
        }
    }

    pub fn state_mut(&mut self) -> &mut dyn State {
        self.state.as_mut()
    }

    pub fn prompt_enabled(&self) -> bool {
        self.state.prompt_enabled()
    }

    pub fn captures_keyboard(&self) -> bool {
        self.state.captures_keyboard()
    }

    pub fn apply(&mut self, transition: StateTransition, session: &mut TerminalSession) -> bool {
        let mut state_changed = false;

        match transition {
            StateTransition::Stay(update) => {
                state_changed |= Self::apply_state_update(update, session);
            }
            StateTransition::Change { mut next, update } => {
                state_changed = true;
                self.state.on_exit();
                state_changed |= Self::apply_state_update(update, session);
                let enter_lines = next.on_enter();
                self.state = next;
                Self::append_output_lines(session, enter_lines);
            }
            StateTransition::Exit(update) => {
                state_changed = true;
                self.state.on_exit();
                state_changed |= Self::apply_state_update(update, session);
                self.state = Box::new(ShellState);
                let enter_lines = self.state.on_enter();
                Self::append_output_lines(session, enter_lines);
            }
        }

        session.sync_state_view(self.state.view(), self.state.uses_alternate_screen());

        state_changed
    }

    fn apply_state_update(update: StateUpdate, session: &mut TerminalSession) -> bool {
        let state_changed = update.changes_view();

        if update.reset_terminal {
            session.reset(bootstrap::initial_screen());
        }

        Self::append_output_lines(session, update.lines);

        for effect in update.effects {
            session.push_effect(effect);
        }

        state_changed
    }

    fn append_output_lines(session: &mut TerminalSession, lines: Vec<String>) {
        if !lines.is_empty() {
            session.append_output(lines);
        }
    }
}
