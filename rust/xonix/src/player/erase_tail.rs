use std::collections::HashMap;
use crate::game::types::{CoordTile, Occupied, Role, Tile};
use super::types::Player;


pub fn erase_tail(
    player: &mut Player, 
    tiles_map: &mut HashMap<CoordTile,Tile>,
) {
    let curr_tile = tiles_map
        .get(&player.curr_pos)
        .expect("No tile found!");

    let curr_is_border_or_claimed = curr_tile.role == Role::Border || curr_tile.role == Role::Claimed;

    if curr_is_border_or_claimed {
        if player.tail.len() == 0 { 
            return 
        }
        player
            .tail
            .iter_mut()
            .for_each(|tile| {
                if let Some(ref_tile) = tiles_map.get_mut(&tile.coord_tile) {
                    ref_tile.occupied = Occupied::Empty;
                    ref_tile.role = Role::Claimed
                }
            });
        player.tail = vec![];
    }
}
