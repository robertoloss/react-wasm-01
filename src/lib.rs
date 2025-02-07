mod utils;
mod rustcode;
use std::{collections::HashMap, sync::Mutex};
use web_sys::{console, CanvasRenderingContext2d};
use lazy_static::lazy_static;
use wasm_bindgen::prelude::*;
use std::fmt::{self,Display};

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}
#[wasm_bindgen]
pub fn event_listener(event_code: &str) {
    //console::log_1(&JsValue::from_str(event_code));
    let player = &mut PLAYER.lock().unwrap();

    match event_code {
        "ArrowDown" => { 
            player.movement = Move::Down
        }
        "ArrowUp" => { 
            player.movement = Move::Up
        }
        "ArrowLeft" => { 
            player.movement = Move::Left
        }
        "ArrowRight" => { 
            player.movement = Move::Right
        }
        _ => { todo!() }
    }
}

pub fn move_player(player: &mut Player, game: &Game) {
    if player.curr_pos != player.prev_pos {
        player.prev_pos = player.curr_pos.clone()
    }
    match player.movement {
        Move::Up => { 
           if player.curr_pos.y > 0 {
                player.curr_pos.y -= 1
            }  
        }
        Move::Down => { 
            if player.curr_pos.y < game.tile_dim.y - 1 { 
                player.curr_pos.y += 1 
            }
        }
        Move::Left => { 
            if player.curr_pos.x > 0 {
                player.curr_pos.x -= 1 
            } 
        }
        Move::Right => { 
            if player.curr_pos.x < game.tile_dim.x - 1 {
                player.curr_pos.x += 1 
            } 
        }
        Move::Still => { }
    }

}

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
struct CoordTile {
    x: u64,
    y: u64
}
impl Display for CoordTile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive(Debug)]
struct CoordAbs {
    x: f64,
    y: f64
}
struct Tile {
    coord_tile: CoordTile,
    coord_abs: CoordAbs,
    role: Role,
    border: bool
}
impl Tile {
   fn get_size() -> f64 { 10. } 
}
enum Role {
    Player,
    Tail,
    Board,
}
#[derive(PartialEq)]
enum Move {
    Up,
    Down,
    Left,
    Right,
    Still
}
pub struct Player {
    pub curr_pos: CoordTile,
    pub prev_pos: CoordTile,
    pub movement: Move
}
impl Player {
    fn new() -> Self {
        Self {
            curr_pos: CoordTile {
                x: 0,
                y: 0
            },
            prev_pos: CoordTile {
                x: 0,
                y: 0
            },
            movement: Move::Still
        }
    }
}

struct Game {
    tile_dim: CoordTile,
    abs_dim: CoordAbs
}
impl Game {
    fn new(dims: CoordTile) -> Self {
        Self {
            tile_dim: dims.clone(),
            abs_dim: CoordAbs {
                x: (dims.x as f64) * Tile::get_size(),
                y: (dims.y as f64) * Tile::get_size()
            }
        }
    }
}

lazy_static! {
    static ref GAME: Mutex<Game> = Mutex::new(Game::new(CoordTile{ x: 80, y: 60}));
    static ref PLAYER: Mutex<Player> = Mutex::new(Player::new());
    static ref TILES_MAP: Mutex<HashMap<CoordTile,Tile>> = Mutex::new(HashMap::new());
    static ref COUNTER: Mutex<u64> = Mutex::new(0);
}


fn create_tiles_map(tiles_map: &mut HashMap<CoordTile,Tile>) {
    let game = GAME.lock().unwrap();
    let mut x = 0;
    let mut y = 0;
    while y < game.abs_dim.y as u64 {
        while x < game.abs_dim.x as u64 {
            let tile = Tile {
                role: Role::Board,
                border: if 
                    x == 0 || 
                    y == 0 || 
                    x / Tile::get_size() as u64 == game.tile_dim.x - 1  || 
                    y / Tile::get_size() as u64 == game.tile_dim.y - 1
                {
                    true
                } else {
                    false
                },
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
    let player = &mut PLAYER.lock().unwrap();
    let game = &mut GAME.lock().unwrap();
    let mut counter = COUNTER.lock().unwrap();


    if *counter > 2 {
        move_player(player, game);
        //console::log_1(&JsValue::from_str(&format!("{}",player.prev_pos)));
        //console::log_1(&JsValue::from_str(&format!("{}",player.curr_pos)));
        if player.curr_pos != player.prev_pos {
            tiles_map
                .get_mut(&player.prev_pos)
                .unwrap_throw()
                .role = Role::Tail;
        } 
        tiles_map
            .get_mut(&player.curr_pos)
            .unwrap_throw()
            .role = Role::Player;
        *counter = 0;
    }
    *counter += 1;

    for (_, tile) in tiles_map.iter() {
        if !tile.border {
            match tile.role {
                Role::Board =>  { ctx.set_fill_style_str("black") }
                Role::Player =>  { ctx.set_fill_style_str("blue") }
                Role::Tail => { ctx.set_fill_style_str("orange") }
            }
        } else {
            if let Role::Player = tile.role {
                ctx.set_fill_style_str("blue");
            } else {
                ctx.set_fill_style_str("gray");
            }
        }
        ctx.fill_rect(
            tile.coord_abs.x, 
            tile.coord_abs.y, 
            Tile::get_size(), 
            Tile::get_size()
        );
    };
}


