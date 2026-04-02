mod bindings;
mod joystick;
mod keyboard;
mod shell;
mod tick;

use std::rc::Rc;

use wasm_bindgen::prelude::JsValue;

use crate::terminal::app::TerminalApp;

pub use bindings::Bindings;

pub fn bind(app: &Rc<TerminalApp>) -> Result<Bindings, JsValue> {
    let window = web_sys::window().ok_or_else(|| JsValue::from_str("window unavailable"))?;
    let mut events = Vec::new();

    events.push(shell::bind_submit(app)?);
    events.push(shell::bind_focus(app)?);
    events.push(keyboard::bind_keyboard(app)?);
    events.extend(joystick::bind_joystick(app)?);

    Ok(Bindings::new(events, tick::bind_tick(app, window)?))
}
