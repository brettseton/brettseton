use std::cell::RefCell;

use wasm_bindgen::prelude::JsValue;

use crate::terminal::model::ViewModel;
use crate::terminal::render_state::RenderState;

use super::diff::{ActiveViewChange, HistoryChange, RenderDiffStrategy};
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

            match plan.history {
                HistoryChange::Noop => {}
                HistoryChange::Append(entries) => {
                    let fragment = self.markup.render_history_entries(entries)?;
                    surface.append_history(&fragment)?;
                }
                HistoryChange::ReplaceAll(entries) => {
                    let fragment = self.markup.render_history_entries(entries)?;
                    surface.replace_history(&fragment)?;
                }
            }

            match plan.active_view {
                ActiveViewChange::Noop => {}
                ActiveViewChange::ReplaceAll(lines) => {
                    let fragment = self.markup.render_app_lines(lines)?;
                    surface.replace_app_view(&fragment)?;
                }
            }

            surface.sync_prompt_state(plan.prompt_enabled);
            surface.sync_focus(plan.prompt_enabled)?;
        }

        self.render_state.borrow_mut().update(
            view.history(),
            view.active_view(),
            view.prompt_enabled(),
        );

        Ok(())
    }
}
