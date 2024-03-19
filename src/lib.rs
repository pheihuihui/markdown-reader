use markdown::{to_html_with_options, Options};
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn markdown_to_html(input: &str) -> String {
    match to_html_with_options(input, &Options::gfm()) {
        Ok(result) => result,
        Err(error) => format!("Error: {}", error),
    }
}
