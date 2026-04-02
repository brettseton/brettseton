use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::JsValue;
use web_sys::{Document, HtmlAnchorElement};

use crate::terminal::model::Effect;

pub struct ExternalLinkOpener {
    document: Document,
}

impl ExternalLinkOpener {
    pub fn new(document: &Document) -> Self {
        Self {
            document: document.clone(),
        }
    }

    pub fn open(&self, href: &str) -> Result<(), JsValue> {
        let anchor = self.document.create_element("a")?;
        let anchor: HtmlAnchorElement = anchor.dyn_into()?;
        anchor.set_href(href);
        anchor.set_target("_blank");
        anchor.set_rel("noopener noreferrer");
        anchor.click();
        Ok(())
    }
}

pub struct TerminalEffectHandler {
    link_opener: ExternalLinkOpener,
}

impl TerminalEffectHandler {
    pub fn new(document: &Document) -> Self {
        Self {
            link_opener: ExternalLinkOpener::new(document),
        }
    }

    pub fn apply(&self, effects: Vec<Effect>) -> Result<(), JsValue> {
        for effect in effects {
            match effect {
                Effect::OpenLink(href) => self.link_opener.open(&href)?,
            }
        }

        Ok(())
    }
}
