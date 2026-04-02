pub mod guess;
pub mod snake;
pub mod tetris;

#[derive(Clone, Default, Eq, PartialEq)]
pub struct AppLine {
    segments: Vec<AppSegment>,
}

#[derive(Clone, Eq, PartialEq)]
pub struct AppSegment {
    text: String,
    class_name: Option<&'static str>,
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum AppKind {
    Guess,
    Snake,
    Tetris,
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum AppMode {
    Interactive,
    Realtime,
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

impl AppLine {
    pub fn text(value: impl Into<String>) -> Self {
        Self {
            segments: vec![AppSegment::plain(value)],
        }
    }

    pub fn segments(&self) -> &[AppSegment] {
        &self.segments
    }

    pub fn push_plain(&mut self, value: impl Into<String>) {
        self.segments.push(AppSegment::plain(value));
    }

    pub fn push_styled(&mut self, value: impl Into<String>, class_name: &'static str) {
        self.segments.push(AppSegment::styled(value, class_name));
    }
}

impl AppSegment {
    pub fn plain(value: impl Into<String>) -> Self {
        Self {
            text: value.into(),
            class_name: None,
        }
    }

    pub fn styled(value: impl Into<String>, class_name: &'static str) -> Self {
        Self {
            text: value.into(),
            class_name: Some(class_name),
        }
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn class_name(&self) -> Option<&'static str> {
        self.class_name
    }
}

pub trait AppRuntime {
    fn handle_command(&mut self, input: &str) -> AppOutput;
    fn on_enter(&mut self) {}
    fn handle_key(&mut self, _key: AppKey) -> Option<AppOutput> {
        None
    }
    fn tick(&mut self) -> Option<AppOutput> {
        None
    }
    fn view(&self) -> Vec<AppLine> {
        Vec::new()
    }
    fn mode(&self) -> AppMode {
        AppMode::Interactive
    }
    fn on_exit(&mut self) {}
}

pub struct AppRegistry;

impl AppRegistry {
    pub fn launch(kind: AppKind) -> (Box<dyn AppRuntime>, Vec<String>) {
        match kind {
            AppKind::Guess => guess::launch(),
            AppKind::Snake => snake::launch(),
            AppKind::Tetris => tetris::launch(),
        }
    }
}

pub(crate) fn lines(values: &[&str]) -> Vec<String> {
    values.iter().map(|value| (*value).to_string()).collect()
}
