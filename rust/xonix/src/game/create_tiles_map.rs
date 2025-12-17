use std::collections::HashMap;
use super::types::{CoordAbs, CoordTile, Occupied, Role, Tile};
use crate::Game;


pub fn create_tiles_map(
    tiles_map: &mut HashMap<CoordTile,Tile>,
    game: &Game,
) {
    let mut x = 0;
    let mut y = 0;
    while y < game.abs_dim.y as u64 {
        while x < game.abs_dim.x as u64 {
            let border = x == 0 || 
                    y == 0 || 
                    x / Tile::get_size() as u64 == game.tile_dim.x - 1  || 
                    y / Tile::get_size() as u64 == game.tile_dim.y - 1;
            let tile = Tile {
                role: if border { Role::Border } else { Role::Board },
                occupied: Occupied::Empty,
                coord_tile: CoordTile {
                    x: x / Tile::get_size() as u64,
                    y: y / Tile::get_size() as u64
                },
                coord_abs: CoordAbs { 
                    x: x as f64, 
                    y: y as f64 
                }
            };
            let tile_coord = tile.coord_tile.clone(); 
            tiles_map.insert(
                tile_coord, 
                tile
            );
            x += Tile::get_size() as u64;
        }
        x = 0;
        y += Tile::get_size() as u64;
    }
}
