use crate::apps::AppLine;

#[derive(Clone, Eq, PartialEq)]
pub struct ScreenLine {
    kind: ScreenLineKind,
    presentation: ScreenLinePresentation,
    segments: Vec<ScreenSegment>,
}

#[derive(Clone, Eq, PartialEq)]
pub struct ScreenSegment {
    text: String,
    class_name: Option<&'static str>,
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum ScreenLineKind {
    Command,
    Output,
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum ScreenLinePresentation {
    Shell,
    App,
}

pub enum Effect {
    OpenLink(String),
}

pub struct ViewModel {
    visible_lines: Vec<ScreenLine>,
    prompt_enabled: bool,
}

impl ScreenLineKind {
    pub fn prefix(self) -> &'static str {
        match self {
            Self::Command => "$",
            Self::Output => ">",
        }
    }

    pub fn class_name(self) -> &'static str {
        match self {
            Self::Command => "line-prefix command",
            Self::Output => "line-prefix output",
        }
    }
}

impl ViewModel {
    pub fn new(visible_lines: Vec<ScreenLine>, prompt_enabled: bool) -> Self {
        Self {
            visible_lines,
            prompt_enabled,
        }
    }

    pub fn visible_lines(&self) -> &[ScreenLine] {
        &self.visible_lines
    }

    pub fn prompt_enabled(&self) -> bool {
        self.prompt_enabled
    }
}

impl ScreenLine {
    pub fn command(text: String) -> Self {
        Self {
            kind: ScreenLineKind::Command,
            presentation: ScreenLinePresentation::Shell,
            segments: vec![ScreenSegment::plain(text)],
        }
    }

    pub fn output(text: String) -> Self {
        Self {
            kind: ScreenLineKind::Output,
            presentation: ScreenLinePresentation::Shell,
            segments: vec![ScreenSegment::plain(text)],
        }
    }

    pub fn from_app_line(line: &AppLine) -> Self {
        Self {
            kind: ScreenLineKind::Output,
            presentation: ScreenLinePresentation::App,
            segments: line
                .segments()
                .iter()
                .map(|segment| ScreenSegment {
                    text: segment.text().to_string(),
                    class_name: segment.class_name(),
                })
                .collect(),
        }
    }

    pub fn kind(&self) -> ScreenLineKind {
        self.kind
    }

    pub fn segments(&self) -> &[ScreenSegment] {
        &self.segments
    }

    pub fn presentation(&self) -> ScreenLinePresentation {
        self.presentation
    }
}

impl ScreenSegment {
    fn plain(text: String) -> Self {
        Self {
            text,
            class_name: None,
        }
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn class_name(&self) -> Option<&'static str> {
        self.class_name
    }
}
