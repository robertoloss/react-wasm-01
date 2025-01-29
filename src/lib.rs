mod utils;
mod rustcode;

use web_sys::{console, CanvasRenderingContext2d};
use wasm_bindgen::prelude::*;
use rustcode::utils::log_out;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}
#[wasm_bindgen]
pub fn event_listener(event_code: &str) {
    console::log_1(&JsValue::from_str(event_code));
}

#[wasm_bindgen]
pub fn render_game(ctx: &CanvasRenderingContext2d) {
    let pixel_size = 1.;
    ctx.set_fill_style_str("gray");
    for y in 0..600 {
        for x in 0..800 {
            ctx.fill_rect(x as f64 * pixel_size, y as f64 * pixel_size, pixel_size, pixel_size);
        }
    }
}
