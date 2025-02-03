mod utils;
mod rustcode;

use std::sync::Mutex;

use web_sys::{console, CanvasRenderingContext2d};
use lazy_static::lazy_static;
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

struct CoordTile {
    x: u64,
    y: u64
}
#[derive(Debug)]
struct CoordAbs {
    x: f64,
    y: f64
}
struct Tile {
    coord_tile: CoordTile,
    coord_abs: CoordAbs,
    player: bool,
    tail: bool
}
impl Tile {
   fn get_size() -> f64 {
        40.
   } 
}

lazy_static! {
    static ref TILES: Mutex<Vec<Tile>> = Mutex::new(vec![]);
}

#[wasm_bindgen]
pub fn game_init() {
    let mut tiles = TILES.lock().unwrap();
    let mut x = 0;
    let mut y = 0;
    while y < 600 {
        while x < 800 {
            let tile = Tile {
                player: false,
                tail: false,
                coord_tile: CoordTile {
                    x: x / Tile::get_size() as u64,
                    y: y / Tile::get_size() as u64
                },
                coord_abs: CoordAbs { 
                    x: x as f64, 
                    y: y as f64 
                }
            };
            tiles.push(tile);
            x += Tile::get_size() as u64;
        }
        x = 0;
        y += Tile::get_size() as u64;
    }
}

#[wasm_bindgen]
pub fn render_game(ctx: &CanvasRenderingContext2d) {
    let tiles = TILES.lock().unwrap();
    ctx.set_fill_style_str("gray");

    tiles.iter().for_each(|tile| {
        if !tile.player && tile.tail {
            ctx.set_fill_style_str("gray");
        }
        //console::log_1(&JsValue::from_str(&format!("{:?}",tile.coord_abs)));
        ctx.fill_rect(
            tile.coord_abs.x, 
            tile.coord_abs.y, 
            Tile::get_size(), 
            Tile::get_size()
        );
    });
}
