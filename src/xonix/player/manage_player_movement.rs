use std::collections::HashMap;
use crate::xonix::game::types::{CoordTile, Game, Occupied, Role, Tile};
use super::{move_player::move_player, types::{Move, Player}};


pub fn manage_player_movement(
    player: &mut Player,
    counter: &mut u64,
    tiles_map: &mut HashMap<CoordTile, Tile>,
    game: &Game
) {
    move_player(player, game, tiles_map);

    if player.curr_pos != player.prev_pos {
        let tile = tiles_map
            .get_mut(&player.prev_pos)
            .expect("No tile found");
        if tile.role != Role::Claimed && tile.role != Role::Border {
            tile.occupied = Occupied::Tail;
            player.tail.push((*tile).clone());
        } else {
            tile.occupied = Occupied::Empty;
        }
    } 
    let curr_tile_role = &tiles_map
        .get(&player.curr_pos)
        .expect("No tile found")
        .occupied;
    if *curr_tile_role == Occupied::Tail || *curr_tile_role == Occupied::Enemy {
        player.curr_pos = player.init_pos.clone();
        tiles_map
            .iter_mut()
            .for_each(|tile| {
                if tile.1.occupied == Occupied::Tail && tile.1.role != Role::Border {
                    tile.1.role = Role::Board;
                    tile.1.occupied = Occupied::Empty;
                }
            });
        player.tail = vec![];
        player.movement = Move::Still;
    }
    tiles_map
        .get_mut(&player.curr_pos)
        .expect("No tile found")
        .occupied = Occupied::Player;
    *counter = 0;
}
