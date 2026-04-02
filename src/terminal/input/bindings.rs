use js_sys::Function;
use wasm_bindgen::JsCast;
use wasm_bindgen::closure::Closure;
use web_sys::{Event, EventTarget, Window};

pub struct Bindings {
    _events: Vec<EventBinding>,
    _interval: IntervalBinding,
}

pub struct EventBinding {
    target: EventTarget,
    event_name: &'static str,
    callback: Function,
    _closure: Closure<dyn FnMut(Event)>,
}

pub struct IntervalBinding {
    window: Window,
    interval_id: i32,
    _closure: Closure<dyn FnMut()>,
}

impl Bindings {
    pub fn new(events: Vec<EventBinding>, interval: IntervalBinding) -> Self {
        Self {
            _events: events,
            _interval: interval,
        }
    }
}

pub fn register_event(
    target: &EventTarget,
    event_name: &'static str,
    closure: Closure<dyn FnMut(Event)>,
) -> Result<EventBinding, wasm_bindgen::JsValue> {
    let callback = closure.as_ref().unchecked_ref::<Function>().clone();
    target.add_event_listener_with_callback(event_name, &callback)?;

    Ok(EventBinding {
        target: target.clone(),
        event_name,
        callback,
        _closure: closure,
    })
}

pub fn interval_binding(
    window: Window,
    interval_id: i32,
    closure: Closure<dyn FnMut()>,
) -> IntervalBinding {
    IntervalBinding {
        window,
        interval_id,
        _closure: closure,
    }
}

impl Drop for EventBinding {
    fn drop(&mut self) {
        let _ = self
            .target
            .remove_event_listener_with_callback(self.event_name, &self.callback);
    }
}

impl Drop for IntervalBinding {
    fn drop(&mut self) {
        self.window.clear_interval_with_handle(self.interval_id);
    }
}
