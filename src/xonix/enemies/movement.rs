use crate::CoordTile;
use super::types::{Enemy, EnemyDir};


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
