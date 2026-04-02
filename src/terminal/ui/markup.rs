use wasm_bindgen::prelude::JsValue;
use web_sys::{Document, DocumentFragment, Element};

use crate::terminal::model::{ScreenLine, ScreenLinePresentation};

pub struct TerminalMarkupRenderer {
    document: Document,
}

impl TerminalMarkupRenderer {
    pub fn new(document: Document) -> Self {
        Self { document }
    }

    pub fn render_screen_lines(&self, lines: &[ScreenLine]) -> Result<DocumentFragment, JsValue> {
        let fragment = self.document.create_document_fragment();
        for line in lines {
            fragment.append_child(&self.render_line(line)?.into())?;
        }
        Ok(fragment)
    }

    fn render_line(&self, line: &ScreenLine) -> Result<Element, JsValue> {
        let row = self.document.create_element("div")?;
        row.set_class_name(match line.presentation() {
            ScreenLinePresentation::Shell => "line",
            ScreenLinePresentation::App => "line app-view",
        });

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

        if line.presentation() == ScreenLinePresentation::Shell {
            let prefix = self.document.create_element("span")?;
            prefix.set_class_name(line.kind().class_name());
            prefix.set_text_content(Some(line.kind().prefix()));
            row.append_child(&prefix)?;
        }
        row.append_child(&content)?;

        Ok(row)
    }
}
