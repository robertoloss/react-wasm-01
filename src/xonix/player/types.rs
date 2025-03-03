use crate::xonix::game::types::{CoordTile, Tile};


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
    pub fn new() -> Self {
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
