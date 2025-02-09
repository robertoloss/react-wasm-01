mod utils;
extern crate console_error_panic_hook;
use std::panic;
mod rustcode;
use std::{collections::HashMap, sync::{Arc, Mutex}};
use rustcode::utils::log_out;
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
        _ => { println!("hey") }
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
pub struct CoordTile {
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
pub struct Tile {
    coord_tile: CoordTile,
    coord_abs: CoordAbs,
    role: Role,
    border: bool
}
impl Tile {
   fn get_size() -> f64 { 10. } 
}
#[derive(Debug)]
enum Role {
    Player,
    Tail,
    Board,
    Claimed
}
#[derive(PartialEq)]
pub enum Move {
    Up,
    Down,
    Left,
    Right,
    Still
}
pub struct Player {
    pub init_pos: CoordTile,
    pub curr_pos: CoordTile,
    pub prev_pos: CoordTile,
    pub movement: Move,
    pub tail: Vec<Arc<Mutex<Tile>>>,
}
impl Player {
    fn new() -> Self {
        let init_pos = CoordTile {
            x: 0,
            y: 0
        };
        Self {
            init_pos: init_pos.clone(),
            curr_pos: init_pos.clone(),
            prev_pos: init_pos.clone(),
            movement: Move::Still,
            tail: vec![]
        }
    }
}

pub struct Game {
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
    static ref TILES_MAP: Mutex<HashMap<CoordTile,Arc<Mutex<Tile>>>> = Mutex::new(HashMap::new());
    static ref COUNTER: Mutex<u64> = Mutex::new(0);
}


fn create_tiles_map(tiles_map: &mut HashMap<CoordTile,Arc<Mutex<Tile>>>) {
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
            let tile_coord = tile.coord_tile.clone(); 
            let mutex_tile = Mutex::new(tile);
            let arc_tile = Arc::new(mutex_tile);
            tiles_map.insert(
                tile_coord, 
                arc_tile
            );
            x += Tile::get_size() as u64;
        }
        x = 0;
        y += Tile::get_size() as u64;
    }
}

#[wasm_bindgen]
pub fn game_init() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    let tiles_map = &mut TILES_MAP.lock().unwrap();
    create_tiles_map(tiles_map);
}

pub fn tile_is_border(tile: &Tile, game: &Game) -> bool {
    tile.coord_tile.x == 0 ||
    tile.coord_tile.y == 0 ||
    tile.coord_tile.x == game.tile_dim.x - 1 ||
    tile.coord_tile.y == game.tile_dim.y - 1
}

pub fn erase_tail(
    player: &mut Player, 
    tiles_map: &mut HashMap<CoordTile,Arc<Mutex<Tile>>>,
    game: &Game
) {
    let curr_tile = tiles_map
        .get(&player.curr_pos)
        .expect("function erase_tail panicked")
        .lock()
        .unwrap();

    //log_out(&format!("{:?}",curr_tile.role));
    let is_border = tile_is_border(&curr_tile, game);
    drop(curr_tile);

    if is_border {
        if player.tail.len() == 0 { return }
        player
            .tail
            .iter()
            .for_each(|mutex_tile| {
                let mut tile = mutex_tile.lock().unwrap();
                tile.role = Role::Claimed;
            });
        player.tail = vec![];
    }
}

pub fn collect_tiles_to_claim() {
    todo!()
}
pub fn claim_tiles() {
    todo!()
}

#[wasm_bindgen]
pub fn render_game(ctx: &CanvasRenderingContext2d) {
    let tiles_map = &mut TILES_MAP.lock().unwrap();
    let player = &mut PLAYER.lock().unwrap();
    let game = &mut GAME.lock().unwrap();
    let mut counter = COUNTER.lock().unwrap();

    if player.tail.len() > 0 {
        erase_tail(player, tiles_map, game);
    }

    if *counter > 3 {
        move_player(player, game);
        if player.curr_pos != player.prev_pos {
            let tile = tiles_map
                .get(&player.prev_pos)
                .expect("render_game panicked");
            let mut locked_tile = tile.lock().unwrap();
            locked_tile.role = Role::Tail;
            player.tail.push(Arc::clone(tile));
        } 
        tiles_map
            .get_mut(&player.curr_pos)
            .unwrap_throw()
            .lock()
            .unwrap()
            .role = Role::Player;
        *counter = 0;
    }
    *counter += 1;

    for (_, tile) in tiles_map.iter() {
        let tile_locked = tile.lock().unwrap();
        if !tile_locked.border {
            match tile_locked.role {
                Role::Board =>  { ctx.set_fill_style_str("black") }
                Role::Player =>  { ctx.set_fill_style_str("yellow") }
                Role::Tail => { ctx.set_fill_style_str("orange") }
                Role::Claimed => { ctx.set_fill_style_str("transparent") }
            }
        } else {
            if let Role::Player = tile_locked.role {
                ctx.set_fill_style_str("blue");
            } else {
                ctx.set_fill_style_str("gray");
            }
        }
        ctx.fill_rect(
            tile_locked.coord_abs.x, 
            tile_locked.coord_abs.y, 
            Tile::get_size(), 
            Tile::get_size()
        );
    };
}


