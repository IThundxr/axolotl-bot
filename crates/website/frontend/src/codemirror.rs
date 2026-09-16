use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::HtmlDivElement;

#[wasm_bindgen(module = "/js/codemirror.js")]
extern "C" {
    #[derive(Clone)]
    pub type EditorView;

    #[wasm_bindgen(js_name = createEditor)]
    pub fn create_editor(parent: HtmlDivElement, text: &str) -> EditorView;

    #[wasm_bindgen(js_name = getText)]
    pub fn get_text(view: &EditorView) -> String;

    #[wasm_bindgen(js_name = setText)]
    pub fn set_text(view: &EditorView, text: &str);

    #[wasm_bindgen(js_name = destroy)]
    pub fn destroy(view: &EditorView);
}