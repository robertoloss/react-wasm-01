use wasm_bindgen::JsValue;
use web_sys::console;


pub fn log_out(s: &str) {
    console::log_1(&JsValue::from_str(s))
}
