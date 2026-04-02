use crate::commands::CommandRegistry;
use crate::terminal::core::states::{StateContext, StateTransition};
use crate::terminal::core::{Action, state_machine::StateMachine};
use crate::terminal::session::TerminalSession;

pub struct ActionDispatcher {
    command_registry: CommandRegistry,
}

impl ActionDispatcher {
    pub fn new() -> Self {
        Self {
            command_registry: CommandRegistry::new(),
        }
    }

    pub fn dispatch(
        &self,
        action: Action,
        state_machine: &mut StateMachine,
        session: &mut TerminalSession,
    ) -> StateTransition {
        match action {
            Action::SubmitCommand(raw) => self.dispatch_command(raw, state_machine, session),
            Action::HandleKey(key) => state_machine.state_mut().handle_key(key),
            Action::Tick => state_machine.state_mut().tick(),
        }
    }

    fn dispatch_command(
        &self,
        raw: String,
        state_machine: &mut StateMachine,
        session: &mut TerminalSession,
    ) -> StateTransition {
        let trimmed = raw.trim().to_string();

        if !state_machine.prompt_enabled() || trimmed.is_empty() {
            return StateTransition::stay();
        }

        session.record_command(&trimmed);
        let context = StateContext::new(&self.command_registry);
        state_machine.state_mut().handle_command(&trimmed, &context)
    }
}
