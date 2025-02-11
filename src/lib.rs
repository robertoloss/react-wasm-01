mod utils;
extern crate console_error_panic_hook;
use std::panic;
mod rustcode;
use std::{collections::HashMap, sync::{Arc, Mutex}};
use std::collections::HashSet;
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

#[derive(Debug,Clone,PartialEq)]
struct CoordAbs {
    x: f64,
    y: f64
}
#[derive(Clone)]
pub struct Tile {
    coord_tile: CoordTile,
    coord_abs: CoordAbs,
    role: Role,
    border: bool
}
impl Tile {
   fn get_size() -> f64 { 5. } 
}
#[derive(Debug,Clone,PartialEq)]
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
    pub tail: Vec<Tile>,
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
    static ref GAME: Mutex<Game> = Mutex::new(Game::new(CoordTile{ x: 80 * 2, y: 60 * 2}));
    static ref PLAYER: Mutex<Player> = Mutex::new(Player::new());
    static ref TILES_MAP: Mutex<HashMap<CoordTile,Tile>> = Mutex::new(HashMap::new());
    static ref COUNTER: Mutex<u64> = Mutex::new(0);
}


fn create_tiles_map() {
    let tiles_map = &mut TILES_MAP.lock().unwrap();
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
            tiles_map.insert(
                tile_coord, 
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
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    create_tiles_map();
}

pub fn tile_is_border(tile: &Tile, game: &Game) -> bool {
    tile.coord_tile.x == 0 ||
    tile.coord_tile.y == 0 ||
    tile.coord_tile.x == game.tile_dim.x - 1 ||
    tile.coord_tile.y == game.tile_dim.y - 1
}

pub fn erase_tail(
    player: &mut Player, 
    tiles_map: &mut HashMap<CoordTile,Tile>,
    game: &Game
) {
    let curr_tile = tiles_map
        .get(&player.curr_pos)
        .expect("function erase_tail panicked");
    let is_border = tile_is_border(&curr_tile, game);
    if is_border {
        if player.tail.len() == 0 { return }
        player
            .tail
            .iter_mut()
            .for_each(|tile| {
                if let Some(ref_tile) = tiles_map.get_mut(&tile.coord_tile) {
                    ref_tile.role = Role::Claimed;
                }
            });
        player.tail = vec![];
    }
}

pub fn collect_tiles_to_claim(
    player: &mut Player,
    tiles_map: &mut HashMap<CoordTile, Tile>,
    game: &Game,
    curr_tile: &Tile,  
    visited: &mut Vec<CoordTile>,  
) {
    if curr_tile.role != Role::Board || visited.contains(&curr_tile.coord_tile) {
        return;  
    }

    visited.push(curr_tile.coord_tile.clone());  

    if let Some(ref_tile) = tiles_map.get_mut(&curr_tile.coord_tile) {
        ref_tile.role = Role::Claimed;  
    }
    let directions = [
        (-1, 0), (1, 0),
        (0, -1), (0, 1),  
    ];

    for (dx, dy) in directions {
        let new_x = curr_tile.coord_tile.x as i64 + dx;
        let new_y = curr_tile.coord_tile.y as i64 + dy;

        if new_x >= 0 && new_x < (game.tile_dim.x - (1 as u64)) as i64
            && new_y >= 0 && new_y < (game.tile_dim.y - (1 as u64)) as i64
        {
            if let Some(new_tile) = tiles_map.get(&CoordTile {
                x: new_x as u64,
                y: new_y as u64,
            }) {
                let tile = new_tile.clone();
                collect_tiles_to_claim(player, tiles_map, game, &tile, visited);
            }
        }
    }
}

pub fn claim_tiles() {
    todo!()
}

#[wasm_bindgen]
pub fn render_game(ctx: &CanvasRenderingContext2d) {
    let mut tiles_map = TILES_MAP.lock().unwrap();
    let mut player = PLAYER.lock().unwrap();
    let game = GAME.lock().unwrap();
    let mut counter = COUNTER.lock().unwrap();

    if player.tail.len() > 0 {
        let tail_count = player
            .tail
            .iter()
            .filter(|tile| !tile.border)
            .count();
        log_out(&format!("tail count: {}", tail_count));
        erase_tail(&mut player, &mut tiles_map, &game);
        let curr_tile = tiles_map
            .get(&player.curr_pos)
            .expect("function erase_tail panicked");
        if tail_count > 0 && curr_tile.border {
            log_out("tail non empty at the border");
            let tile_init = tiles_map
                .get(&CoordTile {
                    x: curr_tile.coord_tile.x - 1,
                    y: curr_tile.coord_tile.y + 1
                })
                .expect("No tile found");
            let tile_initial = tile_init.clone();
            let mut visited: Vec<CoordTile> = vec![];

            collect_tiles_to_claim(
                &mut player, 
                &mut tiles_map, 
                &game, 
                &tile_initial, 
                &mut visited
            );
        }
    }

    if *counter > 1 {
        move_player(&mut player, &game);
        if player.curr_pos != player.prev_pos {
            let tile = tiles_map
                .get_mut(&player.prev_pos)
                .expect("No tile found");
            tile.role = Role::Tail;
            player.tail.push((*tile).clone());
        } 
        tiles_map
            .get_mut(&player.curr_pos)
            .expect("No tile found")
            .role = Role::Player;
        *counter = 0;
    }
    *counter += 1;

    for (_, tile) in tiles_map.iter() {
        if !tile.border {
            match tile.role {
                Role::Board =>  { ctx.set_fill_style_str("black") }
                Role::Player =>  { ctx.set_fill_style_str("yellow") }
                Role::Tail => { ctx.set_fill_style_str("orange") }
                Role::Claimed => { ctx.set_fill_style_str("transparent") }
            }
        } else {
            if let Role::Player = tile.role {
                ctx.set_fill_style_str("blue");
            } else {
                ctx.set_fill_style_str("transparent");
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


