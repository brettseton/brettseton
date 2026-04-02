use crate::apps::AppLine;
use crate::terminal::model::Entry;

#[derive(Default)]
pub(crate) struct RenderState {
    history: Vec<Entry>,
    active_view: Vec<AppLine>,
    prompt_enabled: bool,
}

impl RenderState {
    pub(crate) fn history(&self) -> &[Entry] {
        &self.history
    }

    pub(crate) fn active_view(&self) -> &[AppLine] {
        &self.active_view
    }

    pub(crate) fn update(
        &mut self,
        history: &[Entry],
        active_view: &[AppLine],
        prompt_enabled: bool,
    ) {
        self.history = history.to_vec();
        self.active_view = active_view.to_vec();
        self.prompt_enabled = prompt_enabled;
    }
}
