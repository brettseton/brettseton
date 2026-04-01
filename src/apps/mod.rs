pub mod guess;
pub mod snake;

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum AppKind {
    Guess,
    Snake,
}

impl AppKind {
    pub fn launch(self) -> (Box<dyn AppRuntime>, Vec<String>) {
        match self {
            Self::Guess => guess::launch(),
            Self::Snake => snake::launch(),
        }
    }
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum AppKey {
    Up,
    Down,
    Left,
    Right,
    Escape,
}

pub enum AppOutput {
    Continue(Vec<String>),
    Exit(Vec<String>),
}

pub trait AppRuntime {
    fn handle_command(&mut self, input: &str) -> AppOutput;
    fn handle_key(&mut self, _key: AppKey) -> Option<AppOutput> {
        None
    }
    fn tick(&mut self) -> Option<AppOutput> {
        None
    }
    fn view(&self) -> Vec<String> {
        Vec::new()
    }
    fn is_realtime(&self) -> bool {
        false
    }
}
