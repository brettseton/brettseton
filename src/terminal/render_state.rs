use crate::terminal::model::ScreenLine;

#[derive(Default)]
pub(crate) struct RenderState {
    visible_lines: Vec<ScreenLine>,
}

impl RenderState {
    pub(crate) fn visible_lines(&self) -> &[ScreenLine] {
        &self.visible_lines
    }

    pub(crate) fn update(&mut self, visible_lines: &[ScreenLine], _prompt_enabled: bool) {
        self.visible_lines = visible_lines.to_vec();
    }
}
