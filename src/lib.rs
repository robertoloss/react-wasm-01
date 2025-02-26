mod utils;
use std::panic;
mod rustcode;
use std::{collections::HashMap, sync::Mutex};
use std::collections::HashSet;
use rustcode::utils::log_out;
use web_sys::CanvasRenderingContext2d;
use lazy_static::lazy_static;
use wasm_bindgen::prelude::*;
use std::fmt::{self, format, Display};
use web_time::SystemTime;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[derive(Clone)]
pub enum EnemyDir {
    UpRight,
    UpLeft,
    DownRight,
    DownLeft
}
pub struct Enemy {
    timer: u64,
    pos_tile: CoordTile,
    prev_pos_tile: CoordTile,
    direction: EnemyDir,
}
impl Enemy {
    fn moves(self: &mut Self, tiles_map: &mut HashMap<CoordTile, Tile>) {
        let new_pos_tile = tile_enemy_moves_to(self);
        //log_out(&format!("new_pos_tile {:?}", new_pos_tile));
        let new_pos_role = tiles_map
            .get(&new_pos_tile)
            .expect(&format!(
                "Enemy moves: no tile found at {} {:?}",
                new_pos_tile,
                tiles_map
                    .get(&self.pos_tile)
                    .expect("Z")
                .role
            ))
            .role
            .clone();

        if new_pos_role == Role::Claimed || new_pos_role == Role::Border {
            let curr_dir = self.direction.clone();
            match curr_dir {
                EnemyDir::DownLeft => {
                    if check_if_tile_empty(
                        self.pos_tile.x - 1, 
                        self.pos_tile.y - 1, 
                        tiles_map
                    ) { self.direction = EnemyDir::UpLeft}
                    else if check_if_tile_empty(
                        self.pos_tile.x + 1, 
                        self.pos_tile.y + 1, 
                        tiles_map
                    ) { self.direction = EnemyDir::DownRight }
                    else { self.direction = EnemyDir::UpRight }
                }
                EnemyDir::DownRight => {
                    if check_if_tile_empty(
                        self.pos_tile.x + 1, 
                        self.pos_tile.y - 1, 
                        tiles_map
                    ) { self.direction = EnemyDir::UpRight}
                    else if check_if_tile_empty(
                        self.pos_tile.x - 1, 
                        self.pos_tile.y + 1, 
                        tiles_map
                    ) { self.direction = EnemyDir::DownLeft }
                    else { self.direction = EnemyDir::UpLeft }
                }
                EnemyDir::UpLeft => {
                    if check_if_tile_empty(
                        self.pos_tile.x + 1, 
                        self.pos_tile.y - 1, 
                        tiles_map
                    ) { self.direction = EnemyDir::UpRight; }
                    else if check_if_tile_empty(
                        self.pos_tile.x - 1, 
                        self.pos_tile.y + 1, 
                        tiles_map
                    ) { self.direction = EnemyDir::DownLeft; }
                    else { self.direction = EnemyDir::DownRight }
                }
                EnemyDir::UpRight => {
                    if check_if_tile_empty(
                        self.pos_tile.x - 1, 
                        self.pos_tile.y - 1, 
                        tiles_map
                    ) { self.direction = EnemyDir::UpLeft}
                    else if check_if_tile_empty(
                        self.pos_tile.x + 1, 
                        self.pos_tile.y + 1, 
                        tiles_map
                    ) { self.direction = EnemyDir::DownRight }
                    else { self.direction = EnemyDir::DownLeft }
                }
            }
        }

        if self.pos_tile != self.prev_pos_tile {
            self.prev_pos_tile = self.pos_tile.clone()
        }

        match &self.direction {
            EnemyDir::UpRight => {
                self.pos_tile.x += 1;
                self.pos_tile.y -= 1;
            }
            EnemyDir::UpLeft => {
                self.pos_tile.x -= 1;
                self.pos_tile.y -= 1;
            } 
            EnemyDir::DownRight => {
                self.pos_tile.x += 1;
                self.pos_tile.y += 1;
            }
            EnemyDir::DownLeft => {
                self.pos_tile.x -= 1;
                self.pos_tile.y += 1;
            }
        }
    }
}

pub fn check_if_tile_empty(x: u64, y: u64, tiles_map: &mut HashMap<CoordTile, Tile>) -> bool {
    tiles_map
        .get(&CoordTile { x, y })
        .expect("check_if_tile_empty: no tile found")
        .role != Role::Border
    &&
    tiles_map
        .get(&CoordTile { x, y })
        .expect("check_if_tile_empty: no tile found")
        .role != Role::Claimed
}

pub fn tile_enemy_moves_to(enemy: &Enemy) -> CoordTile {
    let mut tile = CoordTile {
        x: enemy.pos_tile.x,
        y: enemy.pos_tile.y
    };
    match enemy.direction {
        EnemyDir::UpRight => {
            tile.x += 1;
            tile.y -= 1;
        }
        EnemyDir::UpLeft => {
            tile.x -= 1;
            tile.y -= 1;
        } 
        EnemyDir::DownRight => {
            tile.x += 1;
            tile.y += 1;
        }
        EnemyDir::DownLeft => {
            tile.x -= 1;
            tile.y += 1;
        }
    }
    tile
}

#[wasm_bindgen]
pub fn event_listener(event_code: &str) {
    //console::log_1(&JsValue::from_str(event_code));
    let player = &mut PLAYER.lock().unwrap();

    match event_code {
        "ArrowDown" => { 
            player.movement = Move::Down;
            player.moving = true;
        }
        "ArrowUp" => { 
            player.movement = Move::Up;
            player.moving = true;
        }
        "ArrowLeft" => { 
            player.movement = Move::Left;
            player.moving = true;
        }
        "ArrowRight" => { 
            player.movement = Move::Right;
            player.moving = true;
        }
        _ => { println!("hey") }
    }
}

pub fn move_player(
    player: &mut Player, 
    game: &Game,
    tiles_map: &mut HashMap<CoordTile,Tile>
) {
    if player.curr_pos != player.prev_pos {
        player.prev_pos = player.curr_pos.clone()
    }
    let curr_tile = tiles_map.get(&player.curr_pos);
    match player.movement {
        Move::Up => { 
           if player.curr_pos.y > 0 {
                player.moving = true;
                player.curr_pos.y -= 1
            } else {
                player.moving = false;
            } 
        }
        Move::Down => { 
            if player.curr_pos.y < game.tile_dim.y - 1 { 
                player.moving = true;
                player.curr_pos.y += 1 
            } else {
                player.moving = false;
            }
        }
        Move::Left => { 
            if player.curr_pos.x > 0 {
                player.moving = true;
                player.curr_pos.x -= 1 
            } else {
                player.moving = false;
            } 
        }
        Move::Right => { 
            if player.curr_pos.x < game.tile_dim.x - 1 {
                player.moving = true;
                player.curr_pos.x += 1 
            } else {
                player.moving = false;
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
#[derive(Clone,Debug)]
pub struct Tile {
    coord_tile: CoordTile,
    coord_abs: CoordAbs,
    role: Role,
    occupied: Occupied,
}
impl Tile {
   fn get_size() -> f64 { 8. } // 8. 
}

#[derive(Debug,Clone,PartialEq)]
enum Role {
    Border,
    Board,
    Claimed,
}

#[derive(Debug,Clone,PartialEq)]
enum Occupied {
    Player,
    Enemy,
    Tail,
    Empty
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
    pub moving: bool,
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
            tail: vec![],
            moving: false,
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
    static ref GAME: Mutex<Game> = Mutex::new(Game::new(CoordTile{ 
        x: (( 8. / Tile::get_size()) * 100.) as u64, 
        y: (( 8. / Tile::get_size()) * 75.) as u64
    })); // 100 75
    static ref PLAYER: Mutex<Player> = Mutex::new(Player::new());
    static ref TILES_MAP: Mutex<HashMap<CoordTile,Tile>> = Mutex::new(HashMap::new());
    static ref COUNTER: Mutex<u64> = Mutex::new(0);
    static ref LAST_TIME: Mutex<f64> = Mutex::new(0.0);
    static ref ENEMIES: Mutex<Vec<Enemy>> = Mutex::new(vec![]);
}

fn create_tiles_map() {
    let tiles_map = &mut TILES_MAP.lock().unwrap();
    let game = GAME.lock().unwrap();
    let mut x = 0;
    let mut y = 0;
    while y < game.abs_dim.y as u64 {
        while x < game.abs_dim.x as u64 {
            let border = x == 0 || 
                    y == 0 || 
                    x / Tile::get_size() as u64 == game.tile_dim.x - 1  || 
                    y / Tile::get_size() as u64 == game.tile_dim.y - 1;
            let mut tile = Tile {
                role: if border { Role::Border } else { Role::Board },
                occupied: Occupied::Empty,
                coord_tile: CoordTile {
                    x: x / Tile::get_size() as u64,
                    y: y / Tile::get_size() as u64
                },
                coord_abs: CoordAbs { 
                    x: x as f64, 
                    y: y as f64 
                }
            };
            //if tile.coord_tile.x == 40 && tile.coord_tile.y == 20 {
            //    tile.occupied = Occupied::Enemy;
            //}
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
    let pos = CoordTile { x: 6, y: 6, };
    let pos2 = CoordTile { x: 23, y: 27, };
    let new_enemy = Enemy {
        timer: 0,
        pos_tile: pos.clone(),
        prev_pos_tile: pos,
        direction: EnemyDir::DownRight
    };
    let new_enemy2 = Enemy {
        timer: 0,
        pos_tile: pos2.clone(),
        prev_pos_tile: pos2,
        direction: EnemyDir::UpLeft
    };
    let mut enemies = ENEMIES.lock().unwrap();
    enemies.push(new_enemy);
    enemies.push(new_enemy2);
}

pub fn erase_tail(
    player: &mut Player, 
    tiles_map: &mut HashMap<CoordTile,Tile>,
) {
    let curr_tile = tiles_map
        .get(&player.curr_pos)
        .expect("No tile found!");

    let curr_is_border_or_claimed = curr_tile.role == Role::Border || curr_tile.role == Role::Claimed;

    if curr_is_border_or_claimed {
        if player.tail.len() == 0 { 
            log_out("no tail");
            return 
        }
        player
            .tail
            .iter_mut()
            .for_each(|tile| {
                if let Some(ref_tile) = tiles_map.get_mut(&tile.coord_tile) {
                    ref_tile.occupied = Occupied::Empty;
                    ref_tile.role = Role::Claimed
                }
            });
        player.tail = vec![];
    }
}


pub fn collect_tiles_to_claim(
    tiles_map: &mut HashMap<CoordTile, Tile>,
    game: &Game,
    start_tile: &Tile, 
) 
    -> Vec<CoordTile>
{
    let mut visited: HashSet<CoordTile> = HashSet::new();
    let mut stack: Vec<CoordTile> = Vec::new();
    let mut tiles_to_claim: Vec<CoordTile> = vec![];

    stack.push(start_tile.coord_tile.clone());

    while let Some(curr_coord) = stack.pop() {
        if !visited.insert(curr_coord.clone()) {
            continue;
        }
        if let Some(ref_tile) = tiles_map.get_mut(&curr_coord) {
            if ref_tile.occupied == Occupied::Enemy {
                tiles_to_claim = vec![];
                break
            }
            if ref_tile.occupied == Occupied::Tail || ref_tile.occupied == Occupied::Player {
                continue;
            }
            if ref_tile.role == Role::Border {
                tiles_to_claim.push(ref_tile.coord_tile.clone());
                continue
            }
            if ref_tile.role == Role::Claimed { continue }
            tiles_to_claim.push(ref_tile.coord_tile.clone());
        }
        let directions = [
            (-1, 0), (1, 0),
            (0, -1), (0, 1),  
        ];
        for (dx, dy) in directions {
            let new_x = curr_coord.x as i64 + dx;
            let new_y = curr_coord.y as i64 + dy;

            if new_x >= 0 && new_x < (game.tile_dim.x - 1) as i64
                && new_y >= 0 && new_y < (game.tile_dim.y - 1) as i64
            {
                let new_coord = CoordTile {
                    x: new_x as u64,
                    y: new_y as u64,
                };

                if !visited.contains(&new_coord) {
                    stack.push(new_coord);
                }
            }
        }
    }
    tiles_to_claim
}

pub fn claim_tiles(
    tiles_map: &mut HashMap<CoordTile,Tile>,
    tiles_to_claim: &Vec<CoordTile>,
) {
    tiles_to_claim
        .into_iter()
        .for_each(|coord| {
            if let Some(tile) = tiles_map.get_mut(&coord) {
                tile.role = Role::Claimed;
            }
        });
}

#[wasm_bindgen]
pub fn render_game(ctx: &CanvasRenderingContext2d) {
    let mut tiles_map = TILES_MAP.lock().unwrap();
    let mut player = PLAYER.lock().unwrap();
    let game = GAME.lock().unwrap();
    let mut counter = COUNTER.lock().unwrap();
    let mut enemies = ENEMIES.lock().unwrap();

    let mut last_time = LAST_TIME.lock().unwrap();
    let mut now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("A temporal paradox occurred!")
        .as_secs_f64();

    while now - *last_time < 0.017 {
        now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("time went backwards")
            .as_secs_f64();
    } 
    *last_time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("time went backwards")
        .as_secs_f64();

    //log_out(&format!("tail: {}", player.tail.len()));


    for enemy in enemies.iter_mut() {
        if enemy.timer < 2 {
            enemy.timer += 1;
            continue
        }
        enemy.timer = 0;
        enemy.moves(&mut tiles_map);
        tiles_map
            .get_mut(&enemy.pos_tile)
            .expect("Enemies iter in render: no tile found")
            .occupied = Occupied::Enemy;
        tiles_map
            .get_mut(&enemy.prev_pos_tile)
            .expect("Enemies iter in render: no tile found")
            .occupied = Occupied::Empty;
    };
    

    if player.tail.len() > 0 {
        let tail_count = player
            .tail
            .iter()
            .filter(|tile| tile.role != Role::Border)
            .count();

        let curr_tile = tiles_map
            .get(&player.curr_pos)
            .expect("function erase_tail panicked");

        let curr_is_border_or_claimed = curr_tile.role == Role::Border || curr_tile.role == Role::Claimed;

        if tail_count > 0 && curr_is_border_or_claimed {
            player.movement = Move::Still;

            let mut tiles_init: Vec<Tile> = vec![];

            let directions = [
                (-1, -1), (1, -1),
                (-1, 1), (1, 1),  
            ];
            //log_out(&format!("curr_tile: {}, {}", curr_tile.coord_tile.x, curr_tile.coord_tile.y));

            for (dx, dy) in directions {
                let new_x = curr_tile.coord_tile.x as i64 + dx;
                let new_y = curr_tile.coord_tile.y as i64 + dy;

                if new_x >= 0 && new_x < (game.tile_dim.x - 1) as i64
                    && new_y >= 0 && new_y < (game.tile_dim.y - 1) as i64
                {
                    let tile_init = tiles_map.get(
                        &CoordTile {
                            x: new_x as u64,
                            y: new_y as u64
                        })
                        .expect("No tile found");
                    let tile_init = tile_init.clone();
                    tiles_init.push(tile_init);
                }
            }
            tiles_init
                .into_iter()
                .for_each(|tile| {
                    let tiles_to_claim = collect_tiles_to_claim(
                        &mut tiles_map, 
                        &game, 
                        &tile, 
                    );
                    claim_tiles(&mut tiles_map, &tiles_to_claim);
                });
        }
        erase_tail(&mut player, &mut tiles_map);
    }

    if *counter > 1 {
        move_player(&mut player, &game, &mut tiles_map);

        if player.curr_pos != player.prev_pos {
            let tile = tiles_map
                .get_mut(&player.prev_pos)
                .expect("No tile found");
            if tile.role != Role::Claimed && tile.role != Role::Border {
                tile.occupied = Occupied::Tail;
                player.tail.push((*tile).clone());
            } else {
                tile.occupied = Occupied::Empty;
            }
        } 
        let curr_tile_role = &tiles_map
            .get(&player.curr_pos)
            .expect("No tile found")
            .occupied;
        if *curr_tile_role == Occupied::Tail || *curr_tile_role == Occupied::Enemy {
            player.curr_pos = player.init_pos.clone();
            tiles_map
                .iter_mut()
                .for_each(|tile| {
                    if tile.1.occupied == Occupied::Tail && tile.1.role != Role::Border {
                        tile.1.role = Role::Board;
                        tile.1.occupied = Occupied::Empty;
                    }
                });
            player.tail = vec![];
            player.movement = Move::Still;
        }
        tiles_map
            .get_mut(&player.curr_pos)
            .expect("No tile found")
            .occupied = Occupied::Player;
        *counter = 0;
    }
    *counter += 1;

    for (_, tile) in tiles_map.iter() {
        match tile.role {
            Role::Board =>  { ctx.set_fill_style_str("black") }
            Role::Claimed => { ctx.set_fill_style_str("transparent") }
            Role::Border => { ctx.set_fill_style_str("transparent");}
        }
        match tile.occupied {
            Occupied::Player => { ctx.set_fill_style_str("blue") }
            Occupied::Tail => { ctx.set_fill_style_str("orange") }
            Occupied::Enemy => { ctx.set_fill_style_str("red") }
            Occupied::Empty => { }
        }
        ctx.fill_rect(
            tile.coord_abs.x, 
            tile.coord_abs.y, 
            Tile::get_size(), 
            Tile::get_size()
        );
    };
}


