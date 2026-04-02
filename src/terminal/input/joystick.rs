use std::cell::RefCell;
use std::rc::Rc;

use wasm_bindgen::JsCast;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::prelude::JsValue;
use web_sys::{Event, PointerEvent};

use crate::apps::AppKey;
use crate::terminal::app::TerminalApp;
use crate::terminal::core::Action;

use super::bindings::{EventBinding, register_event};

const MAX_RADIUS: f64 = 26.0;
const INPUT_THRESHOLD: f64 = 10.0;

pub fn bind_joystick(app: &Rc<TerminalApp>) -> Result<Vec<EventBinding>, JsValue> {
    let active_pointer = Rc::new(RefCell::new(None::<i32>));
    let mut events = Vec::new();

    events.push(bind_pointer_down(app, &active_pointer)?);
    events.push(bind_pointer_move(app, &active_pointer)?);

    for event_name in ["pointerup", "pointercancel", "lostpointercapture"] {
        events.push(bind_pointer_end(app, &active_pointer, event_name)?);
    }

    Ok(events)
}

fn bind_pointer_down(
    app: &Rc<TerminalApp>,
    active_pointer: &Rc<RefCell<Option<i32>>>,
) -> Result<EventBinding, JsValue> {
    let app_for_down = Rc::clone(app);
    let pointer_for_down = Rc::clone(active_pointer);
    let on_down = Closure::wrap(Box::new(move |event: Event| {
        let event: PointerEvent = event.unchecked_into();
        if !app_for_down.captures_keyboard() {
            return;
        }

        event.prevent_default();
        *pointer_for_down.borrow_mut() = Some(event.pointer_id());
        let _ = app_for_down.capture_control_pointer(event.pointer_id());
        let _ = update_joystick(&app_for_down, &event);
    }) as Box<dyn FnMut(_)>);

    register_event(&app.control_stick_target(), "pointerdown", on_down)
}

fn bind_pointer_move(
    app: &Rc<TerminalApp>,
    active_pointer: &Rc<RefCell<Option<i32>>>,
) -> Result<EventBinding, JsValue> {
    let app_for_move = Rc::clone(app);
    let pointer_for_move = Rc::clone(active_pointer);
    let on_move = Closure::wrap(Box::new(move |event: Event| {
        let event: PointerEvent = event.unchecked_into();
        if *pointer_for_move.borrow() != Some(event.pointer_id()) {
            return;
        }

        event.prevent_default();
        let _ = update_joystick(&app_for_move, &event);
    }) as Box<dyn FnMut(_)>);

    register_event(&app.control_stick_target(), "pointermove", on_move)
}

fn bind_pointer_end(
    app: &Rc<TerminalApp>,
    active_pointer: &Rc<RefCell<Option<i32>>>,
    event_name: &'static str,
) -> Result<EventBinding, JsValue> {
    let app_for_up = Rc::clone(app);
    let pointer_for_up = Rc::clone(active_pointer);
    let on_up = Closure::wrap(Box::new(move |event: Event| {
        let event: PointerEvent = event.unchecked_into();
        if *pointer_for_up.borrow() != Some(event.pointer_id()) {
            return;
        }

        event.prevent_default();
        *pointer_for_up.borrow_mut() = None;
        let _ = app_for_up.release_control_pointer(event.pointer_id());
        let _ = app_for_up.reset_control_thumb();
    }) as Box<dyn FnMut(_)>);

    register_event(&app.control_stick_target(), event_name, on_up)
}

fn update_joystick(app: &Rc<TerminalApp>, event: &PointerEvent) -> Result<(), JsValue> {
    let (dx, dy, magnitude) = joystick_vector(app, event);
    let (clamped_x, clamped_y) = clamp_vector(dx, dy, magnitude);

    app.set_control_thumb_offset(clamped_x, clamped_y)?;

    if magnitude < INPUT_THRESHOLD {
        return Ok(());
    }

    let _ = app.dispatch(Action::HandleKey(resolve_direction(dx, dy)))?;
    Ok(())
}

fn joystick_vector(app: &Rc<TerminalApp>, event: &PointerEvent) -> (f64, f64, f64) {
    let rect = app.control_stick_rect();
    let center_x = rect.x() + rect.width() / 2.0;
    let center_y = rect.y() + rect.height() / 2.0;
    let dx = event.client_x() as f64 - center_x;
    let dy = event.client_y() as f64 - center_y;
    let magnitude = (dx * dx + dy * dy).sqrt();
    (dx, dy, magnitude)
}

fn clamp_vector(dx: f64, dy: f64, magnitude: f64) -> (f64, f64) {
    let scale = if magnitude > MAX_RADIUS {
        MAX_RADIUS / magnitude
    } else {
        1.0
    };

    (dx * scale, dy * scale)
}

fn resolve_direction(dx: f64, dy: f64) -> AppKey {
    if dx.abs() > dy.abs() {
        if dx > 0.0 {
            AppKey::Right
        } else {
            AppKey::Left
        }
    } else if dy > 0.0 {
        AppKey::Down
    } else {
        AppKey::Up
    }
}
