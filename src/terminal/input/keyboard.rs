use std::rc::Rc;

use wasm_bindgen::JsCast;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::prelude::JsValue;
use web_sys::Event;

use crate::apps::AppKey;
use crate::terminal::app::TerminalApp;
use crate::terminal::core::Action;

use super::bindings::{EventBinding, register_event};

pub fn bind_keyboard(app: &Rc<TerminalApp>) -> Result<EventBinding, JsValue> {
    let app_for_keydown = Rc::clone(app);
    let on_keydown = Closure::wrap(Box::new(move |event: Event| {
        let event: web_sys::KeyboardEvent = event.unchecked_into();
        if app_for_keydown.captures_keyboard() {
            event.prevent_default();

            if let Some(key) = map_app_key(event.key().as_str()) {
                let _ = app_for_keydown.dispatch(Action::HandleKey(key));
            }
            return;
        }

        let Some(key) = map_app_key(event.key().as_str()) else {
            return;
        };

        if app_for_keydown
            .dispatch(Action::HandleKey(key))
            .unwrap_or(false)
        {
            event.prevent_default();
        }
    }) as Box<dyn FnMut(_)>);

    register_event(&app.shell_target(), "keydown", on_keydown)
}

pub fn map_app_key(value: &str) -> Option<AppKey> {
    match value {
        "ArrowUp" | "w" | "W" => Some(AppKey::Up),
        "ArrowDown" | "s" | "S" => Some(AppKey::Down),
        "ArrowLeft" | "a" | "A" => Some(AppKey::Left),
        "ArrowRight" | "d" | "D" => Some(AppKey::Right),
        "Escape" => Some(AppKey::Escape),
        _ => None,
    }
}
