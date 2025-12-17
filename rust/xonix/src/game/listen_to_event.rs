use crate::{player::types::Move, PAUSE, PLAYER};

pub fn listen_to_event(event_code: &str) {
    //console::log_1(&JsValue::from_str(event_code));
    let player = &mut PLAYER.lock().unwrap();
    let mut pause = PAUSE.lock().unwrap();

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
        "KeyP" => {
            *pause = if *pause { false } else { true };
        }
        _ => { println!("hey") }
    }
}
