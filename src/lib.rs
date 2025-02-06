mod utils;
mod rustcode;
use std::{collections::HashMap, sync::Mutex};
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
    let player = &mut PLAYER.lock().unwrap();

    match event_code {
        "ArrowDown" => { 
            player.curr_pos.y += 1; 
        }
        "ArrowUp" => { 
            player.curr_pos.y -= 1; 
        }
        "ArrowLeft" => { 
            player.curr_pos.x -= 1; 
        }
        "ArrowRight" => { 
            player.curr_pos.x += 1; 
        }
        _ => { todo!() }
    }
}
#[derive(Eq, PartialEq, Hash, Clone)]
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
    role: Role
}
impl Tile {
   fn get_size() -> f64 { 10. } 
}
enum Role {
    Player,
    Tail,
    Board,
}
struct Player {
    curr_pos: CoordTile
}
impl Player {
    fn new() -> Self {
        Self {
            curr_pos: CoordTile {
                x: 0,
                y: 0
            }
        }
    }
}

struct Game {
    tiles_map: HashMap<CoordTile,Tile>,
    player: Player
}
impl Game {
    fn new() -> Self {
        Self {
            tiles_map: HashMap::new(),
            player: Player::new(),
        }
    }
    
}

lazy_static! {
    static ref GAME: Mutex<Game> = Mutex::new(Game::new());
    static ref PLAYER: Mutex<Player> = Mutex::new(Player::new());
    static ref TILES_MAP: Mutex<HashMap<CoordTile,Tile>> = Mutex::new(HashMap::new());
}
fn create_tiles_map(tiles_map: &mut HashMap<CoordTile,Tile>) {
    let mut x = 0;
    let mut y = 0;
    while y < 600 {
        while x < 800 {
            let tile = Tile {
                role: Role::Board, 
                coord_tile: CoordTile {
                    x: x / Tile::get_size() as u64,
                    y: y / Tile::get_size() as u64
                },
                coord_abs: CoordAbs { 
                    x: x as f64, 
                    y: y as f64 
                }
            };
            tiles_map.insert(
                tile.coord_tile.clone(), 
                tile
            );
            x += Tile::get_size() as u64;
        }
        x = 0;
        y += Tile::get_size() as u64;
    }
}

#[wasm_bindgen]
pub fn game_init() {
    let tiles_map = &mut TILES_MAP.lock().unwrap();
    create_tiles_map(tiles_map);
}

#[wasm_bindgen]
pub fn render_game(ctx: &CanvasRenderingContext2d) {
    let tiles_map = &mut TILES_MAP.lock().unwrap();
    let player = PLAYER.lock().unwrap();
    tiles_map.get_mut(&player.curr_pos).unwrap_throw().role = Role::Player;

    for (_, tile) in tiles_map.iter() {
        match tile.role {
            Role::Board =>  { ctx.set_fill_style_str("black") }
            Role::Player =>  { ctx.set_fill_style_str("transparent") }
            _ => todo!()
        }
        ctx.fill_rect(
            tile.coord_abs.x, 
            tile.coord_abs.y, 
            Tile::get_size(), 
            Tile::get_size()
        );
    };
}


