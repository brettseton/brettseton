mod apps;
mod commands;
mod terminal;
mod terminal_fs;

use std::cell::RefCell;
use std::rc::Rc;

use apps::AppKey;
use terminal::core::TerminalCore;
use terminal::renderer::TerminalRenderer;
use terminal_fs::load_terminal_fs;
use wasm_bindgen::JsCast;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    let window = web_sys::window().ok_or_else(|| JsValue::from_str("window unavailable"))?;
    let document = window
        .document()
        .ok_or_else(|| JsValue::from_str("document unavailable"))?;

    let renderer = TerminalRenderer::new(document)?;
    let core = TerminalCore::new(load_terminal_fs());
    let app = App::new(renderer, core);
    app.mount()?;

    Ok(())
}

struct App {
    core: RefCell<TerminalCore>,
    renderer: TerminalRenderer,
}

impl App {
    fn new(renderer: TerminalRenderer, core: TerminalCore) -> Self {
        renderer.set_social_links(core.links());
        Self {
            core: RefCell::new(core),
            renderer,
        }
    }

    fn mount(self) -> Result<(), JsValue> {
        self.render()?;

        let app = Rc::new(self);

        {
            let app_for_submit = Rc::clone(&app);
            let on_submit = Closure::wrap(Box::new(move |event: web_sys::Event| {
                event.prevent_default();
                if let Err(_error) = app_for_submit.submit_command() {
                    // web_sys::console::error_1(&_error);
                }
            }) as Box<dyn FnMut(_)>);

            app.renderer
                .form_node
                .add_event_listener_with_callback("submit", on_submit.as_ref().unchecked_ref())?;
            on_submit.forget();
        }

        {
            let app_for_click = Rc::clone(&app);
            let on_click = Closure::wrap(Box::new(move |_event: web_sys::Event| {
                if !app_for_click.core.borrow().captures_keyboard() {
                    let _ = app_for_click.renderer.focus_input();
                }
            }) as Box<dyn FnMut(_)>);

            app.renderer
                .shell_node
                .add_event_listener_with_callback("click", on_click.as_ref().unchecked_ref())?;
            on_click.forget();
        }

        {
            let app_for_keydown = Rc::clone(&app);
            let on_keydown = Closure::wrap(Box::new(move |event: web_sys::KeyboardEvent| {
                if app_for_keydown.core.borrow().captures_keyboard() {
                    event.prevent_default();

                    if let Some(key) = map_app_key(event.key().as_str()) {
                        let _ = app_for_keydown.handle_key(key);
                    }
                    return;
                }

                let Some(key) = map_app_key(event.key().as_str()) else {
                    return;
                };

                if app_for_keydown.handle_key(key).unwrap_or(false) {
                    event.prevent_default();
                }
            }) as Box<dyn FnMut(_)>);

            app.renderer
                .shell_node
                .add_event_listener_with_callback("keydown", on_keydown.as_ref().unchecked_ref())?;
            on_keydown.forget();
        }

        {
            let app_for_tick = Rc::clone(&app);
            let on_tick = Closure::wrap(Box::new(move || {
                let _ = app_for_tick.tick();
            }) as Box<dyn FnMut()>);

            web_sys::window()
                .ok_or_else(|| JsValue::from_str("window unavailable"))?
                .set_interval_with_callback_and_timeout_and_arguments_0(
                    on_tick.as_ref().unchecked_ref(),
                    120,
                )?;
            on_tick.forget();
        }

        Ok(())
    }

    fn submit_command(&self) -> Result<(), JsValue> {
        let value = self.renderer.take_input_value();
        let effects = self.core.borrow_mut().run_command(value);
        self.render()?;
        self.renderer.apply_effects(effects)
    }

    fn handle_key(&self, key: AppKey) -> Result<bool, JsValue> {
        if self.core.borrow_mut().handle_key(key) {
            self.render()?;
            return Ok(true);
        }

        Ok(false)
    }

    fn tick(&self) -> Result<(), JsValue> {
        if self.core.borrow_mut().tick() {
            self.render()?;
        }

        Ok(())
    }

    fn render(&self) -> Result<(), JsValue> {
        let core = self.core.borrow();
        self.renderer
            .render(core.history(), core.active_view(), !core.captures_keyboard())
    }
}

fn map_app_key(value: &str) -> Option<AppKey> {
    match value {
        "ArrowUp" | "w" | "W" => Some(AppKey::Up),
        "ArrowDown" | "s" | "S" => Some(AppKey::Down),
        "ArrowLeft" | "a" | "A" => Some(AppKey::Left),
        "ArrowRight" | "d" | "D" => Some(AppKey::Right),
        "Escape" => Some(AppKey::Escape),
        _ => None,
    }
}
