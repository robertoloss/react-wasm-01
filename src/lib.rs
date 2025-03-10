use std::panic;
mod xonix;
use std::{collections::HashMap, sync::Mutex};
use xonix::enemies::create_spheres::create_spheres;
use xonix::enemies::move_spheres::move_spheres;
use xonix::enemies::types_sphere::Sphere;
use xonix::game::create_tiles_map::create_tiles_map;
use xonix::game::delta_wait::delta_wait;
use xonix::game::draw::draw;
use xonix::game::listen_to_event::listen_to_event;
use xonix::game::types::{CoordTile, Game, Tile};
use xonix::player::manage_border_hit::manage_border_hit;
use xonix::player::manage_player_movement::manage_player_movement;
use xonix::player::types::Player;
use web_sys::CanvasRenderingContext2d;
use lazy_static::lazy_static;
use wasm_bindgen::prelude::*;
use xonix::utils::{log_out, pause_game};


lazy_static! {
    static ref GAME: Mutex<Game> = Mutex::new(Game::new(CoordTile{ 
        x: (( 8. / Tile::get_size()) * 100.) as u64, 
        y: (( 8. / Tile::get_size()) * 75.) as u64
    })); // 100 75
    static ref PLAYER: Mutex<Player> = Mutex::new(Player::new());
    static ref TILES_MAP: Mutex<HashMap<CoordTile,Tile>> = Mutex::new(HashMap::new());
    static ref COUNTER: Mutex<u64> = Mutex::new(0);
    static ref LAST_TIME: Mutex<f64> = Mutex::new(0.0);
    static ref SPHERES: Mutex<Vec<Sphere>> = Mutex::new(vec![]);
    static ref PAUSE: Mutex<bool> = Mutex::new(false);
}

#[wasm_bindgen]
pub fn event_listener(event_code: &str) {
    listen_to_event(event_code);
}

#[wasm_bindgen]
pub fn game_init() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    let game = GAME.lock().unwrap();
    let mut tiles_map = TILES_MAP.lock().unwrap();
    let mut spheres = SPHERES.lock().unwrap();

    create_tiles_map(
        &mut tiles_map,
        &game
    );
    create_spheres(
        &mut spheres,
        &tiles_map
    );
}

#[wasm_bindgen]
pub fn render_game(ctx: &CanvasRenderingContext2d) {
    let game = GAME.lock().unwrap();
    let mut tiles_map = TILES_MAP.lock().unwrap();
    let mut player = PLAYER.lock().unwrap();
    let mut counter = COUNTER.lock().unwrap();
    let mut last_time = LAST_TIME.lock().unwrap();
    let mut spheres = SPHERES.lock().unwrap();
    let pause = PAUSE.lock().unwrap();
    
    delta_wait(&mut last_time);

    if !*pause { 
        move_spheres(
            &mut spheres,
            &mut tiles_map
        );

        if player.tail.len() > 0 {
            manage_border_hit(
                &mut player, 
                &mut tiles_map, 
                &game
            );
        }

        if *counter >= 2 {
            manage_player_movement(
                &mut player, 
                &mut counter, 
                &mut tiles_map, 
                &game
            );
        }
        *counter += 1;
    }

    draw(
        &mut tiles_map, 
        ctx,
        &spheres
    );
}


