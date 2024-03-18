use markdown::to_html;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn markdown_to_html_default(input: &str) -> String {
    to_html(input)
}
