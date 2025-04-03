use std::collections::HashMap;
use crate::game::types::{CoordTile, Game, Tile};
use super::types::{Move, Player};

pub fn move_player(
    player: &mut Player, 
    game: &Game,
    tiles_map: &mut HashMap<CoordTile,Tile>
) {
    if player.curr_pos != player.prev_pos {
        player.prev_pos = player.curr_pos.clone()
    }
    let curr_tile = tiles_map.get(&player.curr_pos);
    match player.movement {
        Move::Up => { 
           if player.curr_pos.y > 0 {
                player.moving = true;
                player.curr_pos.y -= 1
            } else {
                player.moving = false;
            } 
        }
        Move::Down => { 
            if player.curr_pos.y < game.tile_dim.y - 1 { 
                player.moving = true;
                player.curr_pos.y += 1 
            } else {
                player.moving = false;
            }
        }
        Move::Left => { 
            if player.curr_pos.x > 0 {
                player.moving = true;
                player.curr_pos.x -= 1 
            } else {
                player.moving = false;
            } 
        }
        Move::Right => { 
            if player.curr_pos.x < game.tile_dim.x - 1 {
                player.moving = true;
                player.curr_pos.x += 1 
            } else {
                player.moving = false;
            }
        }
        Move::Still => { }
    }

}

