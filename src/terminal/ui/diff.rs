use crate::apps::AppLine;
use crate::terminal::model::{Entry, ViewModel};
use crate::terminal::render_state::RenderState;

pub enum HistoryChange<'a> {
    Noop,
    Append(&'a [Entry]),
    ReplaceAll(&'a [Entry]),
}

pub enum ActiveViewChange<'a> {
    Noop,
    ReplaceAll(&'a [AppLine]),
}

pub struct RenderPlan<'a> {
    pub history: HistoryChange<'a>,
    pub active_view: ActiveViewChange<'a>,
    pub prompt_enabled: bool,
}

pub struct RenderDiffStrategy;

impl RenderDiffStrategy {
    pub fn build<'a>(&self, previous: &RenderState, next: &'a ViewModel) -> RenderPlan<'a> {
        let history = if !is_history_extension(previous.history(), next.history()) {
            HistoryChange::ReplaceAll(next.history())
        } else if previous.history().len() == next.history().len() {
            HistoryChange::Noop
        } else {
            HistoryChange::Append(&next.history()[previous.history().len()..])
        };

        let active_view = if previous.active_view() == next.active_view() {
            ActiveViewChange::Noop
        } else {
            ActiveViewChange::ReplaceAll(next.active_view())
        };

        RenderPlan {
            history,
            active_view,
            prompt_enabled: next.prompt_enabled(),
        }
    }
}

fn is_history_extension(previous: &[Entry], next: &[Entry]) -> bool {
    previous.len() <= next.len() && previous.iter().zip(next.iter()).all(|(a, b)| a == b)
}
