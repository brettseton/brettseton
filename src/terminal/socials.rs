use std::collections::BTreeMap;

use wasm_bindgen::prelude::JsValue;
use web_sys::{Document, HtmlAnchorElement};

use crate::terminal::dom::element_by_id;

pub struct SocialLinks {
    anchors: BTreeMap<String, HtmlAnchorElement>,
}

impl SocialLinks {
    pub fn new(document: &Document) -> Result<Self, JsValue> {
        let mut anchors = BTreeMap::new();
        anchors.insert(
            "links/github".to_string(),
            element_by_id::<HtmlAnchorElement>(document, "social-github")?,
        );
        anchors.insert(
            "links/linkedin".to_string(),
            element_by_id::<HtmlAnchorElement>(document, "social-linkedin")?,
        );

        Ok(Self { anchors })
    }

    pub fn apply(&self, links: &BTreeMap<String, String>) {
        for (path, anchor) in &self.anchors {
            if let Some(href) = links.get(path) {
                anchor.set_href(href);
            }
        }
    }
}
