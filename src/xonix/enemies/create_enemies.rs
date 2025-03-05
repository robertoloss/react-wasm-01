use crate::xonix::game::types::CoordTile;
use super::types::{Enemy, EnemyDir};

pub fn create_enemies(enemies: &mut Vec<Enemy>) {
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
    enemies.push(new_enemy);
    enemies.push(new_enemy2);
}
