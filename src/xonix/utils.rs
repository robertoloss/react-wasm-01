use std::fmt::Debug;
use wasm_bindgen::JsValue;
use web_sys::console;

use crate::PAUSE;


#[allow(dead_code)]
pub fn log_out<T: Debug>(s: T) {
    console::log_1(&JsValue::from_str(&format!("{:?}",s)))
}


#[allow(dead_code)]
pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

pub fn pause_game(pause: &mut bool) {
    if *pause {
        return;
    }
}
