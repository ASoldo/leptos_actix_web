use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = dayjs)]
    pub fn dayjs() -> Dayjs;

    #[wasm_bindgen]
    pub type Dayjs;

    #[wasm_bindgen(method)]
    pub fn format(this: &Dayjs, fmt: &str) -> String;
}
