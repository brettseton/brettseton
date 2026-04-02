mod apps;
mod commands;
mod terminal;

use std::cell::RefCell;

use terminal::app::MountedTerminalApp;
use terminal::app::TerminalApp;
use terminal::core::TerminalCore;
use terminal::link::TerminalEffectHandler;
use terminal::ui::TerminalUi;
use wasm_bindgen::prelude::*;

thread_local! {
    static TERMINAL_APP: RefCell<Option<MountedTerminalApp>> = const { RefCell::new(None) };
}

#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    let window = web_sys::window().ok_or_else(|| JsValue::from_str("window unavailable"))?;
    let document = window
        .document()
        .ok_or_else(|| JsValue::from_str("document unavailable"))?;

    let ui = TerminalUi::new(document.clone())?;
    let effects = TerminalEffectHandler::new(&document);
    let core = TerminalCore::new();
    let app = TerminalApp::new(ui, effects, core);
    let mounted = app.mount()?;

    TERMINAL_APP.with(|slot| {
        *slot.borrow_mut() = Some(mounted);
    });

    Ok(())
}
