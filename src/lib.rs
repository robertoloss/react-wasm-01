use std::panic;
mod xonix;
use std::{collections::HashMap, sync::Mutex};
use xonix::enemies::create_enemies::create_enemies;
use xonix::enemies::types::Enemy;
use xonix::game::claim_tiles::claim_tiles;
use xonix::game::collect_tiles_to_claim::collect_tiles_to_claim;
use xonix::game::create_tiles_map::create_tiles_map;
use xonix::game::delta_wait::delta_wait;
use xonix::game::types::{CoordTile, Game, Occupied, Role, Tile};
use xonix::player::erase_tail::erase_tail;
use xonix::player::move_player::move_player;
use xonix::player::types::{Move, Player};
use web_sys::CanvasRenderingContext2d;
use lazy_static::lazy_static;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
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

#[wasm_bindgen]
pub fn game_init() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    let game = GAME.lock().unwrap();
    let mut tiles_map = TILES_MAP.lock().unwrap();
    let mut enemies = ENEMIES.lock().unwrap();

    create_tiles_map(
        &mut tiles_map,
        &game
    );
    create_enemies(&mut enemies);
}


#[wasm_bindgen]
pub fn render_game(ctx: &CanvasRenderingContext2d) {
    let game = GAME.lock().unwrap();
    let mut tiles_map = TILES_MAP.lock().unwrap();
    let mut player = PLAYER.lock().unwrap();
    let mut counter = COUNTER.lock().unwrap();
    let mut enemies = ENEMIES.lock().unwrap();
    let mut last_time = LAST_TIME.lock().unwrap();
    
    delta_wait(&mut last_time);

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

    if *counter >= 2 {
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


