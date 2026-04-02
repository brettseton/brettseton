mod action_dispatcher;
mod state_machine;
mod states;

use crate::apps::AppKey;
use crate::terminal::bootstrap;
use crate::terminal::model::{Effect, ViewModel};
use crate::terminal::session::TerminalSession;

use self::action_dispatcher::ActionDispatcher;
use self::state_machine::StateMachine;

pub enum Action {
    SubmitCommand(String),
    HandleKey(AppKey),
    Tick,
}

#[derive(Default)]
pub struct DispatchResult {
    effects: Vec<Effect>,
    state_changed: bool,
}

pub struct TerminalCore {
    session: TerminalSession,
    dispatcher: ActionDispatcher,
    state_machine: StateMachine,
}

impl TerminalCore {
    pub fn new() -> Self {
        Self {
            session: TerminalSession::new(bootstrap::initial_screen()),
            dispatcher: ActionDispatcher::new(),
            state_machine: StateMachine::new(),
        }
    }

    pub fn captures_keyboard(&self) -> bool {
        self.state_machine.captures_keyboard()
    }

    pub fn prompt_enabled(&self) -> bool {
        self.state_machine.prompt_enabled()
    }

    pub fn dispatch(&mut self, action: Action) -> DispatchResult {
        let transition =
            self.dispatcher
                .dispatch(action, &mut self.state_machine, &mut self.session);
        let state_changed = self.state_machine.apply(transition, &mut self.session);

        DispatchResult {
            effects: self.session.take_effects(),
            state_changed,
        }
    }

    pub fn view(&self) -> ViewModel {
        self.session.view(self.state_machine.prompt_enabled())
    }
}

impl DispatchResult {
    pub fn effects(self) -> Vec<Effect> {
        self.effects
    }

    pub fn state_changed(&self) -> bool {
        self.state_changed
    }
}
