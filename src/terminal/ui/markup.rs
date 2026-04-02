use wasm_bindgen::prelude::JsValue;
use web_sys::{Document, DocumentFragment, Element};

use crate::apps::AppLine;
use crate::terminal::model::{Entry, EntryKind};

pub struct TerminalMarkupRenderer {
    document: Document,
}

impl TerminalMarkupRenderer {
    pub fn new(document: Document) -> Self {
        Self { document }
    }

    pub fn render_history_entries(&self, entries: &[Entry]) -> Result<DocumentFragment, JsValue> {
        let fragment = self.document.create_document_fragment();
        for entry in entries {
            fragment.append_child(&self.render_entry(entry)?.into())?;
        }
        Ok(fragment)
    }

    pub fn render_app_lines(&self, lines: &[AppLine]) -> Result<DocumentFragment, JsValue> {
        let fragment = self.document.create_document_fragment();
        for line in lines {
            fragment.append_child(&self.render_app_line(line)?.into())?;
        }
        Ok(fragment)
    }

    fn render_entry(&self, entry: &Entry) -> Result<Element, JsValue> {
        let row = self.document.create_element("div")?;
        row.set_class_name("line");

        let prefix = self.document.create_element("span")?;
        prefix.set_class_name(entry.kind.class_name());
        prefix.set_text_content(Some(entry.kind.prefix()));

        row.append_child(&prefix)?;
        row.append_child(&self.document.create_text_node(&entry.text))?;

        Ok(row)
    }

    fn render_app_line(&self, line: &AppLine) -> Result<Element, JsValue> {
        let row = self.document.create_element("div")?;
        row.set_class_name("line app-view");

        let prefix = self.document.create_element("span")?;
        prefix.set_class_name(EntryKind::Output.class_name());
        prefix.set_text_content(Some(EntryKind::Output.prefix()));

        let content = self.document.create_element("span")?;
        for segment in line.segments() {
            if let Some(class_name) = segment.class_name() {
                let span = self.document.create_element("span")?;
                span.set_class_name(class_name);
                span.set_text_content(Some(segment.text()));
                content.append_child(&span)?;
            } else {
                content.append_child(&self.document.create_text_node(segment.text()))?;
            }
        }

        row.append_child(&prefix)?;
        row.append_child(&content)?;

        Ok(row)
    }
}
