use std::collections::HashMap;
use super::types::{CoordTile, Role, Tile};


pub fn claim_tiles(
    tiles_map: &mut HashMap<CoordTile,Tile>,
    tiles_to_claim: &Vec<CoordTile>,
) {
    tiles_to_claim
        .into_iter()
        .for_each(|coord| {
            if let Some(tile) = tiles_map.get_mut(&coord) {
                tile.role = Role::Claimed;
            }
        });
}
