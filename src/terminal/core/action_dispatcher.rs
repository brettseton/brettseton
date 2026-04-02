use crate::commands::{CommandContext, aliases as command_aliases, names as command_names};
use crate::terminal::core::states::{CommandCatalog, StateContext, StateTransition};
use crate::terminal::core::{Action, state_machine::StateMachine};
use crate::terminal::session::TerminalSession;

pub struct ActionDispatcher {
    command_catalog: CommandCatalog,
}

impl ActionDispatcher {
    pub fn new() -> Self {
        Self {
            command_catalog: CommandCatalog {
                names: command_names().collect(),
                aliases: command_aliases().collect(),
            },
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
        let context = StateContext::new(session.terminal_fs(), &self.command_catalog);
        state_machine.state_mut().handle_command(&trimmed, &context)
    }
}

impl StateContext<'_> {
    pub fn command_context(&self) -> CommandContext<'_> {
        CommandContext::new(
            self.terminal_fs,
            &self.command_catalog.names,
            &self.command_catalog.aliases,
        )
    }
}
