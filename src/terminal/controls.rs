use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::JsValue;
use web_sys::{Document, EventTarget, HtmlElement};

use crate::terminal::dom::element_by_id;

pub struct AppControls {
    controls_node: HtmlElement,
    control_stick: HtmlElement,
    control_thumb: HtmlElement,
}

impl AppControls {
    pub fn new(document: &Document) -> Result<Self, JsValue> {
        Ok(Self {
            controls_node: element_by_id::<HtmlElement>(document, "app-controls")?,
            control_stick: element_by_id::<HtmlElement>(document, "control-stick")?,
            control_thumb: element_by_id::<HtmlElement>(document, "control-thumb")?,
        })
    }

    pub fn set_hidden(&self, hidden: bool) {
        self.controls_node.set_hidden(hidden);
    }

    pub fn stick_target(&self) -> EventTarget {
        self.control_stick.clone().unchecked_into()
    }

    pub fn stick_rect(&self) -> web_sys::DomRect {
        self.control_stick.get_bounding_client_rect()
    }

    pub fn capture_pointer(&self, pointer_id: i32) -> Result<(), JsValue> {
        self.control_stick.set_pointer_capture(pointer_id)
    }

    pub fn release_pointer(&self, pointer_id: i32) -> Result<(), JsValue> {
        self.control_stick.release_pointer_capture(pointer_id)
    }

    pub fn set_thumb_offset(&self, x: f64, y: f64) -> Result<(), JsValue> {
        self.control_thumb.set_attribute(
            "style",
            &format!("transform: translate(calc(-50% + {x:.1}px), calc(-50% + {y:.1}px));"),
        )
    }

    pub fn reset_thumb(&self) -> Result<(), JsValue> {
        self.control_thumb.remove_attribute("style")?;
        Ok(())
    }
}
