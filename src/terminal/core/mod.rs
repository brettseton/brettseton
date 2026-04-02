mod action_dispatcher;
mod state_machine;
mod states;

use crate::apps::AppKey;
use crate::terminal::bootstrap;
use crate::terminal::model::{Effect, ViewModel};
use crate::terminal::session::TerminalSession;
use crate::terminal_fs::FileSystem;

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
    pub fn new(terminal_fs: Box<dyn FileSystem>) -> Self {
        Self {
            session: TerminalSession::new(terminal_fs, bootstrap::initial_history()),
            dispatcher: ActionDispatcher::new(),
            state_machine: StateMachine::new(),
        }
    }

    pub fn captures_keyboard(&self) -> bool {
        self.state_machine.captures_keyboard()
    }

    pub fn links(&self) -> &std::collections::BTreeMap<String, String> {
        self.session.links()
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
        ViewModel::new(
            self.session.history().to_vec(),
            self.state_machine.view(),
            self.state_machine.prompt_enabled(),
        )
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
