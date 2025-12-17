use wasm_bindgen::prelude::wasm_bindgen;

use crate::PLAYER;
use std::f64::consts::PI;

#[wasm_bindgen]
pub fn event_listener(event: &str) {
    let mut player = PLAYER.lock().unwrap(); 
    //let speed = 8.;
    let num = 50.;
    let div = 10.;

    match event {
        "ArrowUp" => {
            player.pos.x += player.delta.x / div;
            player.pos.y += player.delta.y / div;
        }
        "ArrowDown" => {
            player.pos.x -= player.delta.x / div;
            player.pos.y -= player.delta.y / div;
        }
        "KeyA" => {
            player.pos.x += (player.angle - PI / 2.0).cos() * (num / div);
            player.pos.y += (player.angle - PI / 2.0).sin() * (num / div);
        }
        "KeyD" => {
            player.pos.x += (player.angle + PI / 2.0).cos() * (num / div);
            player.pos.y += (player.angle + PI / 2.0).sin() * (num / div);
        }        
        "ArrowLeft" => { 
            player.angle -= 0.1;
            if player.angle < 0. {
                player.angle += 2.* PI
            }
            player.delta.x = player.angle.cos() * num;
            player.delta.y = player.angle.sin() * num;
        }
        "ArrowRight" => { 
            player.angle += 0.1;
            if player.angle > 2. * PI {
                player.angle -= 2.* PI
            }
            player.delta.x = player.angle.cos() * num;
            player.delta.y = player.angle.sin() * num;
        }
        _ => {}
    }
}
