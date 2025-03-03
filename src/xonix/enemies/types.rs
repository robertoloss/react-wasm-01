use std::collections::HashMap;
use crate::{xonix::game::types::Role, CoordTile, Tile};
use super::{movement::tile_enemy_moves_to, utils::check_if_tile_empty};


#[derive(Clone)]
pub enum EnemyDir {
    UpRight,
    UpLeft,
    DownRight,
    DownLeft
}
pub struct Enemy {
    pub timer: u64,
    pub pos_tile: CoordTile,
    pub prev_pos_tile: CoordTile,
    pub direction: EnemyDir,
}
impl Enemy {
    pub fn moves(self: &mut Self, tiles_map: &mut HashMap<CoordTile, Tile>) {
        let new_pos_tile = tile_enemy_moves_to(self);
        //log_out(&format!("new_pos_tile {:?}", new_pos_tile));
        let new_pos_role = tiles_map
            .get(&new_pos_tile)
            .expect(&format!(
                "Enemy moves: no tile found at {} {:?}",
                new_pos_tile,
                tiles_map
                    .get(&self.pos_tile)
                    .expect("This tile not found!")
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
