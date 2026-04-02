use crate::terminal::model::{ScreenLine, ViewModel};
use crate::terminal::render_state::RenderState;

pub enum VisibleScreenChange<'a> {
    Noop,
    Append(&'a [ScreenLine]),
    ReplaceAll(&'a [ScreenLine]),
}

pub struct RenderPlan<'a> {
    pub visible_screen: VisibleScreenChange<'a>,
    pub prompt_enabled: bool,
}

pub struct RenderDiffStrategy;

impl RenderDiffStrategy {
    pub fn build<'a>(&self, previous: &RenderState, next: &'a ViewModel) -> RenderPlan<'a> {
        let visible_screen =
            if !is_visible_screen_extension(previous.visible_lines(), next.visible_lines()) {
                VisibleScreenChange::ReplaceAll(next.visible_lines())
            } else if previous.visible_lines().len() == next.visible_lines().len() {
                VisibleScreenChange::Noop
            } else {
                VisibleScreenChange::Append(&next.visible_lines()[previous.visible_lines().len()..])
            };

        RenderPlan {
            visible_screen,
            prompt_enabled: next.prompt_enabled(),
        }
    }
}

fn is_visible_screen_extension(previous: &[ScreenLine], next: &[ScreenLine]) -> bool {
    previous.len() <= next.len() && previous.iter().zip(next.iter()).all(|(a, b)| a == b)
}
