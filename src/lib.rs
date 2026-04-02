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
use web_sys::PointerEvent;

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

        bind_joystick(&app)?;

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

fn bind_joystick(app: &Rc<App>) -> Result<(), JsValue> {
    let active_pointer = Rc::new(RefCell::new(None::<i32>));

    {
        let app_for_down = Rc::clone(app);
        let pointer_for_down = Rc::clone(&active_pointer);
        let on_down = Closure::wrap(Box::new(move |event: PointerEvent| {
            if !app_for_down.core.borrow().captures_keyboard() {
                return;
            }

            event.prevent_default();
            *pointer_for_down.borrow_mut() = Some(event.pointer_id());
            let _ = app_for_down
                .renderer
                .control_stick
                .set_pointer_capture(event.pointer_id());
            let _ = update_joystick(&app_for_down, &event);
        }) as Box<dyn FnMut(_)>);

        app.renderer
            .control_stick
            .add_event_listener_with_callback("pointerdown", on_down.as_ref().unchecked_ref())?;
        on_down.forget();
    }

    {
        let app_for_move = Rc::clone(app);
        let pointer_for_move = Rc::clone(&active_pointer);
        let on_move = Closure::wrap(Box::new(move |event: PointerEvent| {
            if *pointer_for_move.borrow() != Some(event.pointer_id()) {
                return;
            }

            event.prevent_default();
            let _ = update_joystick(&app_for_move, &event);
        }) as Box<dyn FnMut(_)>);

        app.renderer
            .control_stick
            .add_event_listener_with_callback("pointermove", on_move.as_ref().unchecked_ref())?;
        on_move.forget();
    }

    {
        let app_for_up = Rc::clone(app);
        let pointer_for_up = Rc::clone(&active_pointer);
        let on_up = Closure::wrap(Box::new(move |event: PointerEvent| {
            if *pointer_for_up.borrow() != Some(event.pointer_id()) {
                return;
            }

            event.prevent_default();
            *pointer_for_up.borrow_mut() = None;
            let _ = app_for_up
                .renderer
                .control_stick
                .release_pointer_capture(event.pointer_id());
            let _ = reset_joystick_thumb(&app_for_up);
        }) as Box<dyn FnMut(_)>);

        for event_name in ["pointerup", "pointercancel", "lostpointercapture"] {
            app.renderer.control_stick.add_event_listener_with_callback(
                event_name,
                on_up.as_ref().unchecked_ref(),
            )?;
        }
        on_up.forget();
    }

    Ok(())
}

fn update_joystick(app: &Rc<App>, event: &PointerEvent) -> Result<(), JsValue> {
    let rect = app.renderer.control_stick.get_bounding_client_rect();
    let center_x = rect.x() + rect.width() / 2.0;
    let center_y = rect.y() + rect.height() / 2.0;
    let dx = event.client_x() as f64 - center_x;
    let dy = event.client_y() as f64 - center_y;

    let max_radius = 26.0;
    let magnitude = (dx * dx + dy * dy).sqrt();
    let scale = if magnitude > max_radius {
        max_radius / magnitude
    } else {
        1.0
    };
    let clamped_x = dx * scale;
    let clamped_y = dy * scale;

    app.renderer.control_thumb.set_attribute(
        "style",
        &format!(
            "transform: translate(calc(-50% + {clamped_x:.1}px), calc(-50% + {clamped_y:.1}px));"
        ),
    )?;

    let threshold = 10.0;
    if magnitude < threshold {
        return Ok(());
    }

    let key = if dx.abs() > dy.abs() {
        if dx > 0.0 {
            AppKey::Right
        } else {
            AppKey::Left
        }
    } else if dy > 0.0 {
        AppKey::Down
    } else {
        AppKey::Up
    };

    let _ = app.handle_key(key)?;
    Ok(())
}

fn reset_joystick_thumb(app: &Rc<App>) -> Result<(), JsValue> {
    app.renderer.control_thumb.remove_attribute("style")?;
    Ok(())
}
