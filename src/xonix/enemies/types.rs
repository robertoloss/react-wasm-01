use std::{collections::HashMap, convert::TryInto};
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
            change_direction(
                self, 
                tiles_map
            );
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

struct Offsets {
    first: Offset,
    second: Offset,
    opposite: EnemyDir
}
struct Offset {
    values: (i64,i64),
    direction: EnemyDir,
}

fn change_direction(
    enemy: &mut Enemy,
    tiles_map: &mut HashMap<CoordTile, Tile>
) {
    let direction = &enemy.direction;
    let offsets: Offsets;
    match direction {
        EnemyDir::DownLeft => {
            let values = (-1,-1);
            offsets = Offsets {
                first: Offset {
                    values,
                    direction: EnemyDir::UpLeft
                },
                second: Offset {
                    values: (-values.0,-values.1),
                    direction: EnemyDir::DownRight
                },
                opposite: EnemyDir::UpRight
            } 
        }
        EnemyDir::DownRight => {
            let values = (1,-1);
            offsets = Offsets {
                first: Offset {
                    values,
                    direction: EnemyDir::UpRight
                },
                second: Offset {
                    values: (-values.0,-values.1),
                    direction: EnemyDir::DownLeft
                },
                opposite: EnemyDir::UpLeft
            } 
        }
        EnemyDir::UpLeft => {
            let values = (-1,1);
            offsets = Offsets {
                first: Offset {
                    values,
                    direction: EnemyDir::DownLeft
                },
                second: Offset {
                    values: (-values.0,-values.1),
                    direction: EnemyDir::UpRight
                },
                opposite: EnemyDir::DownRight
            } 
        }
        EnemyDir::UpRight => {
            let values = (1,1);
            offsets = Offsets {
                first: Offset {
                    values,
                    direction: EnemyDir::DownRight
                },
                second: Offset {
                    values: (-values.0,-values.1),
                    direction: EnemyDir::UpLeft
                },
                opposite: EnemyDir::DownLeft
            } 
        }
    }

    if check_if_tile_empty(
        (enemy.pos_tile.x as i64 + offsets.first.values.0).try_into().unwrap(), 
        (enemy.pos_tile.y as i64 + offsets.first.values.1).try_into().unwrap(), 
        tiles_map
    ) { enemy.direction = offsets.first.direction }
    else if check_if_tile_empty(
        (enemy.pos_tile.x as i64 + offsets.second.values.0).try_into().unwrap(), 
        (enemy.pos_tile.y as i64 + offsets.second.values.1).try_into().unwrap(), 
        tiles_map
    ) { enemy.direction = offsets.second.direction }
    else { enemy.direction = offsets.opposite }
}
