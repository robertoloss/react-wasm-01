use std::collections::HashMap;
use crate::{game::types::Role, CoordTile, Tile};

pub fn check_if_tile_empty(x: u64, y: u64, tiles_map: &mut HashMap<CoordTile, Tile>) -> bool {
    tiles_map
        .get(&CoordTile { x, y })
        .expect("check_if_tile_empty: no tile found")
        .role != Role::Border
    &&
    tiles_map
        .get(&CoordTile { x, y })
        .expect("check_if_tile_empty: no tile found")
        .role != Role::Claimed
}
