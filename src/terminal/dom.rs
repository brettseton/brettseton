use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::JsValue;
use web_sys::Document;

pub(crate) fn element_by_id<T: JsCast>(document: &Document, id: &str) -> Result<T, JsValue> {
    document
        .get_element_by_id(id)
        .ok_or_else(|| JsValue::from_str(&format!("missing element #{id}")))?
        .dyn_into::<T>()
        .map_err(Into::into)
}
