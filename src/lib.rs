use markdown::{
    to_html_with_options, CompileOptions, Constructs, LineEnding, Options, ParseOptions,
};
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn markdown_to_html(input: &str) -> String {
    let option = Options {
        parse: ParseOptions {
            constructs: Constructs {
                attention: true,
                autolink: true,
                block_quote: true,
                character_escape: true,
                character_reference: true,
                code_indented: true,
                code_fenced: true,
                code_text: true,
                definition: true,
                frontmatter: false,
                gfm_autolink_literal: true,
                gfm_footnote_definition: true,
                gfm_label_start_footnote: true,
                gfm_strikethrough: true,
                gfm_table: true,
                gfm_task_list_item: true,
                hard_break_escape: true,
                hard_break_trailing: true,
                heading_atx: true,
                heading_setext: true,
                html_flow: true,
                html_text: true,
                label_start_image: true,
                label_start_link: true,
                label_end: true,
                list_item: true,
                math_flow: true,
                math_text: true,
                mdx_esm: false,
                mdx_expression_flow: false,
                mdx_expression_text: false,
                mdx_jsx_flow: false,
                mdx_jsx_text: false,
                thematic_break: true,
            },
            gfm_strikethrough_single_tilde: false,
            math_text_single_dollar: true,
            mdx_esm_parse: None,
            mdx_expression_parse: None,
        },
        compile: CompileOptions {
            allow_dangerous_html: false,
            allow_dangerous_protocol: false,
            default_line_ending: LineEnding::LineFeed,
            gfm_footnote_label: None,
            gfm_footnote_label_tag_name: None,
            gfm_footnote_label_attributes: None,
            gfm_footnote_back_label: None,
            gfm_footnote_clobber_prefix: None,
            gfm_task_list_item_checkable: false,
            gfm_tagfilter: false,
        },
    };

    match to_html_with_options(input, &option) {
        Ok(result) => result,
        Err(error) => format!("Error: {}", error),
    }
}
