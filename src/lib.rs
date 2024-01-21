mod utils;

use comrak::{markdown_to_html, Options};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn markdown_to_html_default() -> String {
    markdown_to_html("ss", &Options::default())
}