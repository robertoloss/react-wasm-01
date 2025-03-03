use std::collections::HashMap;
use crate::xonix::game::types::{CoordTile, Occupied, Tile};
use super::types::Enemy;


pub fn move_enemies(
    enemies: &mut Vec<Enemy>,
    tiles_map: &mut HashMap<CoordTile, Tile>
) {
    for enemy in enemies.iter_mut() {
        if enemy.timer < 2 {
            enemy.timer += 1;
            continue
        }
        enemy.timer = 0;
        enemy.moves(tiles_map);
        tiles_map
            .get_mut(&enemy.pos_tile)
            .expect("Enemies iter in render: no tile found")
            .occupied = Occupied::Enemy;
        tiles_map
            .get_mut(&enemy.prev_pos_tile)
            .expect("Enemies iter in render: no tile found")
            .occupied = Occupied::Empty;
    };
}
