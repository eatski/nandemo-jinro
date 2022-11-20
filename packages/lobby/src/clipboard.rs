use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen()]
extern "C" {
    #[wasm_bindgen(js_name = "writeClickBoard",js_namespace = ["window","_wasm_js_bridge"])]
    pub fn wirteClickBoard(text: &str);
}
