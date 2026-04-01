use std::collections::BTreeMap;

use wasm_bindgen::JsCast;
use web_sys::{Document, Element, HtmlAnchorElement, HtmlElement, HtmlInputElement, Node};

use crate::terminal::core::{Effect, Entry};

pub struct TerminalRenderer {
    document: Document,
    pub(crate) history_node: HtmlElement,
    pub(crate) form_node: HtmlElement,
    pub(crate) input_node: HtmlInputElement,
    pub(crate) shell_node: HtmlElement,
    socials: BTreeMap<String, HtmlAnchorElement>,
}

impl TerminalRenderer {
    pub fn new(document: Document) -> Result<Self, wasm_bindgen::JsValue> {
        let history_node = element_by_id::<HtmlElement>(&document, "terminal-history")?;
        let form_node = element_by_id::<HtmlElement>(&document, "terminal-form")?;
        let input_node = element_by_id::<HtmlInputElement>(&document, "terminal-input")?;
        let shell_node = element_by_id::<HtmlElement>(&document, "terminal-shell")?;
        shell_node.set_tab_index(0);

        let mut socials = BTreeMap::new();
        socials.insert(
            "links/github".to_string(),
            element_by_id::<HtmlAnchorElement>(&document, "social-github")?,
        );
        socials.insert(
            "links/linkedin".to_string(),
            element_by_id::<HtmlAnchorElement>(&document, "social-linkedin")?,
        );

        Ok(Self {
            document,
            history_node,
            form_node,
            input_node,
            shell_node,
            socials,
        })
    }

    pub fn set_social_links(&self, links: &BTreeMap<String, String>) {
        for (path, anchor) in &self.socials {
            if let Some(href) = links.get(path) {
                anchor.set_href(href);
            }
        }
    }

    pub fn take_input_value(&self) -> String {
        let value = self.input_node.value();
        self.input_node.set_value("");
        value
    }

    pub fn focus_input(&self) -> Result<(), wasm_bindgen::JsValue> {
        self.input_node.focus()
    }

    pub fn focus_shell(&self) -> Result<(), wasm_bindgen::JsValue> {
        self.shell_node.focus()
    }

    pub fn render(
        &self,
        history: &[Entry],
        active_view: &[String],
        prompt_enabled: bool,
    ) -> Result<(), wasm_bindgen::JsValue> {
        self.history_node.set_inner_html("");
        self.input_node.set_disabled(!prompt_enabled);
        if !prompt_enabled {
            self.input_node.set_value("");
        }

        let fragment = self.document.create_document_fragment();
        for entry in history {
            fragment.append_child(&self.render_line(entry)?.into())?;
        }
        for line in active_view {
            fragment.append_child(&self.render_output_line(line)?.into())?;
        }

        self.history_node.append_child(&fragment)?;
        self.history_node
            .append_child(self.form_node.as_ref() as &Node)?;
        self.history_node
            .set_scroll_top(self.history_node.scroll_height());
        if prompt_enabled {
            self.focus_input()?;
        } else {
            self.focus_shell()?;
        }
        Ok(())
    }

    pub fn apply_effects(&self, effects: Vec<Effect>) -> Result<(), wasm_bindgen::JsValue> {
        for effect in effects {
            match effect {
                Effect::OpenLink(href) => self.open_link(&href)?,
            }
        }

        Ok(())
    }

    fn open_link(&self, href: &str) -> Result<(), wasm_bindgen::JsValue> {
        let anchor = self.document.create_element("a")?;
        let anchor: HtmlAnchorElement = anchor.dyn_into()?;
        anchor.set_href(href);
        anchor.set_target("_blank");
        anchor.set_rel("noopener noreferrer");
        anchor.click();
        Ok(())
    }

    fn render_line(&self, entry: &Entry) -> Result<Element, wasm_bindgen::JsValue> {
        let row = self.document.create_element("div")?;
        row.set_class_name("line");

        let prefix = self.document.create_element("span")?;
        prefix.set_class_name(entry.kind.class_name());
        prefix.set_text_content(Some(entry.kind.prefix()));

        row.append_child(&prefix)?;
        row.append_child(&self.document.create_text_node(&entry.text))?;

        Ok(row)
    }

    fn render_output_line(&self, text: &str) -> Result<Element, wasm_bindgen::JsValue> {
        let row = self.document.create_element("div")?;
        row.set_class_name("line app-view");

        let prefix = self.document.create_element("span")?;
        prefix.set_class_name(crate::terminal::core::EntryKind::Output.class_name());
        prefix.set_text_content(Some(crate::terminal::core::EntryKind::Output.prefix()));

        let content = self.document.create_element("span")?;
        content.set_inner_html(&render_app_view_html(text));

        row.append_child(&prefix)?;
        row.append_child(&content)?;

        Ok(row)
    }
}

fn render_app_view_html(text: &str) -> String {
    escape_html(text).replace('*', "<span class=\"snake-food\">*</span>")
}

fn escape_html(value: &str) -> String {
    value.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

fn element_by_id<T: JsCast>(document: &Document, id: &str) -> Result<T, wasm_bindgen::JsValue> {
    document
        .get_element_by_id(id)
        .ok_or_else(|| wasm_bindgen::JsValue::from_str(&format!("missing element #{id}")))?
        .dyn_into::<T>()
        .map_err(Into::into)
}
