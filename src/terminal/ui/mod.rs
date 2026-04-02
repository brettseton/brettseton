mod diff;
mod markup;
mod presenter;
mod surface;

use wasm_bindgen::prelude::JsValue;
use web_sys::{Document, EventTarget};

use crate::terminal::model::ViewModel;

pub use surface::TerminalSurface;

use self::markup::TerminalMarkupRenderer;
use self::presenter::TerminalPresenter;

pub struct TerminalUi {
    surface: TerminalSurface,
    presenter: TerminalPresenter,
}

impl TerminalUi {
    pub fn new(document: Document) -> Result<Self, JsValue> {
        let surface = TerminalSurface::new(&document)?;
        let presenter = TerminalPresenter::new(TerminalMarkupRenderer::new(document));
        Ok(Self { surface, presenter })
    }

    pub fn render(&self, view: &ViewModel) -> Result<(), JsValue> {
        self.presenter.render(&self.surface, view)
    }

    pub fn take_input_value(&self) -> String {
        self.surface.take_input_value()
    }

    pub fn focus_input(&self) -> Result<(), JsValue> {
        self.surface.focus_input()
    }

    pub fn submit_target(&self) -> EventTarget {
        self.surface.submit_target()
    }

    pub fn shell_target(&self) -> EventTarget {
        self.surface.shell_target()
    }

    pub fn control_stick_target(&self) -> EventTarget {
        self.surface.control_stick_target()
    }

    pub fn control_stick_rect(&self) -> web_sys::DomRect {
        self.surface.control_stick_rect()
    }

    pub fn capture_control_pointer(&self, pointer_id: i32) -> Result<(), JsValue> {
        self.surface.capture_control_pointer(pointer_id)
    }

    pub fn release_control_pointer(&self, pointer_id: i32) -> Result<(), JsValue> {
        self.surface.release_control_pointer(pointer_id)
    }

    pub fn set_control_thumb_offset(&self, x: f64, y: f64) -> Result<(), JsValue> {
        self.surface.set_control_thumb_offset(x, y)
    }

    pub fn reset_control_thumb(&self) -> Result<(), JsValue> {
        self.surface.reset_control_thumb()
    }
}
