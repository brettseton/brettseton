use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::JsValue;
use web_sys::{Document, EventTarget, HtmlElement, HtmlInputElement, Node};

use crate::terminal::dom::element_by_id;

pub(crate) struct ShellDom {
    history_node: HtmlElement,
    form_node: HtmlElement,
    input_node: HtmlInputElement,
    shell_node: HtmlElement,
}

impl ShellDom {
    pub(crate) fn new(document: &Document) -> Result<Self, JsValue> {
        let shell_node = element_by_id::<HtmlElement>(document, "terminal-shell")?;
        shell_node.set_tab_index(0);

        Ok(Self {
            history_node: element_by_id::<HtmlElement>(document, "terminal-history")?,
            form_node: element_by_id::<HtmlElement>(document, "terminal-form")?,
            input_node: element_by_id::<HtmlInputElement>(document, "terminal-input")?,
            shell_node,
        })
    }

    pub(crate) fn attach(&self, screen_content: &HtmlElement) -> Result<(), JsValue> {
        self.history_node
            .append_child(screen_content.as_ref() as &Node)?;
        self.history_node
            .append_child(self.form_node.as_ref() as &Node)?;
        Ok(())
    }

    pub(crate) fn take_input_value(&self) -> String {
        let value = self.input_node.value();
        self.input_node.set_value("");
        value
    }

    pub(crate) fn focus_input(&self) -> Result<(), JsValue> {
        self.input_node.focus()
    }

    fn focus_shell(&self) -> Result<(), JsValue> {
        self.shell_node.focus()
    }

    pub(crate) fn set_prompt_enabled(&self, prompt_enabled: bool) {
        self.form_node.set_hidden(!prompt_enabled);
        self.input_node.set_disabled(!prompt_enabled);
        if !prompt_enabled {
            self.input_node.set_value("");
        }
    }

    pub(crate) fn sync_focus(&self, prompt_enabled: bool) -> Result<(), JsValue> {
        self.history_node
            .set_scroll_top(self.history_node.scroll_height());

        if prompt_enabled {
            self.focus_input()
        } else {
            self.focus_shell()
        }
    }

    pub(crate) fn submit_target(&self) -> EventTarget {
        self.form_node.clone().unchecked_into()
    }

    pub(crate) fn shell_target(&self) -> EventTarget {
        self.shell_node.clone().unchecked_into()
    }
}
