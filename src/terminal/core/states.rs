use crate::apps::{AppKey, AppKind, AppLine, AppMode, AppOutput, AppRegistry, AppRuntime};
use crate::commands::{CommandResult, Executable, resolve as resolve_command};
use crate::terminal::model::Effect;
use crate::terminal_fs::FileSystem;

pub struct CommandCatalog {
    pub names: Vec<&'static str>,
    pub aliases: Vec<&'static str>,
}

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
}

pub struct StateContext<'a> {
    pub terminal_fs: &'a dyn FileSystem,
    pub command_catalog: &'a CommandCatalog,
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

pub struct ShellState;

pub struct AppState {
    mode: AppMode,
    runtime: Box<dyn AppRuntime>,
    enter_lines: Vec<String>,
}

impl StateContext<'_> {
    pub fn new<'a>(
        terminal_fs: &'a dyn FileSystem,
        command_catalog: &'a CommandCatalog,
    ) -> StateContext<'a> {
        StateContext {
            terminal_fs,
            command_catalog,
        }
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

impl ShellState {
    fn parse_command<'a>(input: &'a str) -> (&'a str, &'a str) {
        let mut parts = input.split_whitespace();
        let command = parts.next().unwrap_or_default();
        let target = input[command.len()..].trim_start();
        (command, target)
    }

    fn apply_command_result(result: CommandResult) -> StateTransition {
        let mut update = StateUpdate {
            lines: result.lines().to_vec(),
            reset_terminal: result.should_reset_terminal(),
            ..StateUpdate::default()
        };

        if let Some(href) = result.open_link() {
            update.effects.push(Effect::OpenLink(href.to_string()));
        }

        if let Some(kind) = result.launch_app() {
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
        let (command, target) = Self::parse_command(input);
        let command_context = context.command_context();

        match resolve_command(command, context.terminal_fs) {
            Some(Executable::Builtin(command)) => {
                Self::apply_command_result(command.execute(target, &command_context))
            }
            Some(Executable::External(item)) if target.is_empty() => {
                Self::apply_command_result(item.execute(command))
            }
            Some(Executable::External(_)) | None => {
                StateTransition::Stay(StateUpdate::with_lines(vec![if target.is_empty() {
                    format!("{command}: not found")
                } else {
                    format!("{command}: command not found")
                }]))
            }
        }
    }
}

impl AppState {
    fn launch(kind: AppKind) -> Self {
        let (runtime, enter_lines) = AppRegistry::launch(kind);
        let mode = runtime.mode();
        Self {
            mode,
            runtime,
            enter_lines,
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
        std::mem::take(&mut self.enter_lines)
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
        self.runtime.view()
    }

    fn prompt_enabled(&self) -> bool {
        self.mode == AppMode::Interactive
    }

    fn captures_keyboard(&self) -> bool {
        self.mode == AppMode::Realtime
    }
}
