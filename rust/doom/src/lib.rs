mod game;
mod player;
mod draw;
mod map;
mod utils;
use draw::draw_rays::draw_rays;
use draw::main::draw;
use lazy_static::lazy_static;
use player::types::Player;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};
use wasm_bindgen::prelude::*;
use std::panic;
use std::sync::Mutex;

lazy_static! {
    static ref PLAYER: Mutex<Player> = Mutex::new(Player::new()); 
}

#[wasm_bindgen]
pub fn game_init() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
}

#[wasm_bindgen]
pub fn render_game(ctx: &CanvasRenderingContext2d, canvas: &HtmlCanvasElement) {
    let mut player = PLAYER.lock().unwrap(); 

    draw(
        &ctx,
        canvas,
        &mut player
    );

    draw_rays(
        &mut player, 
        &ctx
    );
}
