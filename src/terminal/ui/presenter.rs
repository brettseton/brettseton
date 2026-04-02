use std::cell::RefCell;

use wasm_bindgen::prelude::JsValue;

use crate::terminal::model::ViewModel;
use crate::terminal::render_state::RenderState;

use super::diff::{RenderDiffStrategy, VisibleScreenChange};
use super::markup::TerminalMarkupRenderer;
use super::surface::TerminalSurface;

pub struct TerminalPresenter {
    markup: TerminalMarkupRenderer,
    diff: RenderDiffStrategy,
    render_state: RefCell<RenderState>,
}

impl TerminalPresenter {
    pub fn new(markup: TerminalMarkupRenderer) -> Self {
        Self {
            markup,
            diff: RenderDiffStrategy,
            render_state: RefCell::new(RenderState::default()),
        }
    }

    pub fn render(&self, surface: &TerminalSurface, view: &ViewModel) -> Result<(), JsValue> {
        {
            let state = self.render_state.borrow();
            let plan = self.diff.build(&state, view);

            match plan.visible_screen {
                VisibleScreenChange::Noop => {}
                VisibleScreenChange::Append(lines) => {
                    let fragment = self.markup.render_screen_lines(lines)?;
                    surface.append_screen(&fragment)?;
                }
                VisibleScreenChange::ReplaceAll(lines) => {
                    let fragment = self.markup.render_screen_lines(lines)?;
                    surface.replace_screen(&fragment)?;
                }
            }

            surface.sync_prompt_state(plan.prompt_enabled);
            surface.sync_focus(plan.prompt_enabled)?;
        }

        self.render_state
            .borrow_mut()
            .update(view.visible_lines(), view.prompt_enabled());

        Ok(())
    }
}
