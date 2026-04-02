use std::rc::Rc;

use wasm_bindgen::closure::Closure;
use wasm_bindgen::prelude::JsValue;

use crate::terminal::app::TerminalApp;
use crate::terminal::core::Action;

use super::bindings::{EventBinding, register_event};

pub fn bind_submit(app: &Rc<TerminalApp>) -> Result<EventBinding, JsValue> {
    let app_for_submit = Rc::clone(app);
    let on_submit = Closure::wrap(Box::new(move |event: web_sys::Event| {
        event.prevent_default();
        let value = app_for_submit.take_input_value();
        let _ = app_for_submit.dispatch(Action::SubmitCommand(value));
    }) as Box<dyn FnMut(_)>);

    register_event(&app.submit_target(), "submit", on_submit)
}

pub fn bind_focus(app: &Rc<TerminalApp>) -> Result<EventBinding, JsValue> {
    let app_for_click = Rc::clone(app);
    let on_click = Closure::wrap(Box::new(move |_event: web_sys::Event| {
        let _ = app_for_click.focus_input_if_prompt_enabled();
    }) as Box<dyn FnMut(_)>);

    register_event(&app.shell_target(), "click", on_click)
}
