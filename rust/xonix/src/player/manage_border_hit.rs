use std::collections::HashMap;
use crate::game::{claim_tiles::claim_tiles, collect_tiles_to_claim::collect_tiles_to_claim, types::{CoordTile, Game, Role, Tile}};
use super::{erase_tail::erase_tail, types::{Move, Player}};


pub fn manage_border_hit(
    player: &mut Player,
    tiles_map: &mut HashMap<CoordTile, Tile>,
    game: &Game
) {
    let tail_count = player
        .tail
        .iter()
        .filter(|tile| tile.role != Role::Border)
        .count();

    let curr_tile = tiles_map
        .get(&player.curr_pos)
        .expect("function erase_tail panicked");

    let curr_is_border_or_claimed = curr_tile.role == Role::Border || curr_tile.role == Role::Claimed;

    if tail_count > 0 && curr_is_border_or_claimed {
        player.movement = Move::Still;

        let mut tiles_init: Vec<Tile> = vec![];

        let directions = [
            (-1, -1), (1, -1),
            (-1, 1), (1, 1),  
        ];
        //log_out(&format!("curr_tile: {}, {}", curr_tile.coord_tile.x, curr_tile.coord_tile.y));

        for (dx, dy) in directions {
            let new_x = curr_tile.coord_tile.x as i64 + dx;
            let new_y = curr_tile.coord_tile.y as i64 + dy;

            if new_x >= 0 && new_x < (game.tile_dim.x - 1) as i64
                && new_y >= 0 && new_y < (game.tile_dim.y - 1) as i64
            {
                let tile_init = tiles_map.get(
                    &CoordTile {
                        x: new_x as u64,
                        y: new_y as u64
                    })
                    .expect("No tile found");
                let tile_init = tile_init.clone();
                tiles_init.push(tile_init);
            }
        }
        tiles_init
            .into_iter()
            .for_each(|tile| {
                let tiles_to_claim = collect_tiles_to_claim(
                    tiles_map, 
                    game, 
                    &tile, 
                );
                claim_tiles(tiles_map, &tiles_to_claim);
            });
    }
    erase_tail(player, tiles_map);
}
