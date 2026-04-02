use std::rc::Rc;

use wasm_bindgen::JsCast;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::prelude::JsValue;
use web_sys::Window;

use crate::terminal::app::TerminalApp;
use crate::terminal::core::Action;

use super::bindings::{IntervalBinding, interval_binding};

const TICK_INTERVAL_MS: i32 = 120;

pub fn bind_tick(app: &Rc<TerminalApp>, window: Window) -> Result<IntervalBinding, JsValue> {
    let app_for_tick = Rc::clone(app);
    let on_tick = Closure::wrap(Box::new(move || {
        let _ = app_for_tick.dispatch(Action::Tick);
    }) as Box<dyn FnMut()>);

    let interval_id = window.set_interval_with_callback_and_timeout_and_arguments_0(
        on_tick.as_ref().unchecked_ref(),
        TICK_INTERVAL_MS,
    )?;

    Ok(interval_binding(window, interval_id, on_tick))
}
