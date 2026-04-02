use crate::commands::CommandResult;
use crate::terminal::model::Effect;

use super::{AppState, State, StateContext, StateTransition, StateUpdate};

pub struct ShellState;

impl ShellState {
    fn apply_command_result(result: CommandResult) -> StateTransition {
        let mut update = StateUpdate {
            lines: result.lines().to_vec(),
            reset_terminal: result.should_reset_terminal(),
            ..StateUpdate::default()
        };

        if let Some(href) = result.open_link() {
            update.effects.push(Effect::OpenLink(href.to_string()));
        }

        if let Some(kind) = result.start_program() {
            return StateTransition::Change {
                next: Box::new(AppState::launch(kind)),
                update,
            };
        }

        StateTransition::Stay(update)
    }
}

impl State for ShellState {
    fn handle_command(&mut self, input: &str, context: &StateContext<'_>) -> StateTransition {
        Self::apply_command_result(context.command_registry.execute(input))
    }
}
