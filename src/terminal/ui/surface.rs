use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::JsValue;
use web_sys::{Document, DocumentFragment, EventTarget, HtmlElement, Node};

use crate::terminal::controls::AppControls;
use crate::terminal::shell_dom::ShellDom;

pub struct TerminalSurface {
    shell: ShellDom,
    screen_content: HtmlElement,
    controls: AppControls,
}

impl TerminalSurface {
    pub fn new(document: &Document) -> Result<Self, JsValue> {
        let shell = ShellDom::new(document)?;
        let screen_content = document.create_element("div")?.dyn_into::<HtmlElement>()?;
        let controls = AppControls::new(document)?;
        shell.attach(&screen_content)?;

        Ok(Self {
            shell,
            screen_content,
            controls,
        })
    }

    pub fn take_input_value(&self) -> String {
        self.shell.take_input_value()
    }

    pub fn focus_input(&self) -> Result<(), JsValue> {
        self.shell.focus_input()
    }

    pub fn submit_target(&self) -> EventTarget {
        self.shell.submit_target()
    }

    pub fn shell_target(&self) -> EventTarget {
        self.shell.shell_target()
    }

    pub fn control_stick_target(&self) -> EventTarget {
        self.controls.stick_target()
    }

    pub fn control_stick_rect(&self) -> web_sys::DomRect {
        self.controls.stick_rect()
    }

    pub fn capture_control_pointer(&self, pointer_id: i32) -> Result<(), JsValue> {
        self.controls.capture_pointer(pointer_id)
    }

    pub fn release_control_pointer(&self, pointer_id: i32) -> Result<(), JsValue> {
        self.controls.release_pointer(pointer_id)
    }

    pub fn set_control_thumb_offset(&self, x: f64, y: f64) -> Result<(), JsValue> {
        self.controls.set_thumb_offset(x, y)
    }

    pub fn reset_control_thumb(&self) -> Result<(), JsValue> {
        self.controls.reset_thumb()
    }

    pub fn replace_screen(&self, fragment: &DocumentFragment) -> Result<(), JsValue> {
        self.screen_content.set_inner_html("");
        self.screen_content
            .append_child(fragment.as_ref() as &Node)?;
        Ok(())
    }

    pub fn append_screen(&self, fragment: &DocumentFragment) -> Result<(), JsValue> {
        self.screen_content
            .append_child(fragment.as_ref() as &Node)?;
        Ok(())
    }

    pub fn sync_prompt_state(&self, prompt_enabled: bool) {
        self.shell.set_prompt_enabled(prompt_enabled);
        self.controls.set_hidden(prompt_enabled);
    }

    pub fn sync_focus(&self, prompt_enabled: bool) -> Result<(), JsValue> {
        self.shell.sync_focus(prompt_enabled)
    }
}
