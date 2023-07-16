use wasm_bindgen::prelude::*;
mod parser;

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub unsafe fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}

#[wasm_bindgen]
pub unsafe fn sy(input: &str) -> String {
    return parser::shunting_yard(input.to_string());
}
