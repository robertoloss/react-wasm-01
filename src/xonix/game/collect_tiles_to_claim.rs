use std::collections::{HashMap, HashSet};
use super::types::{CoordTile, Game, Occupied, Role, Tile};


pub fn collect_tiles_to_claim(
    tiles_map: &mut HashMap<CoordTile, Tile>,
    game: &Game,
    start_tile: &Tile, 
) 
    -> Vec<CoordTile>
{
    let mut visited: HashSet<CoordTile> = HashSet::new();
    let mut stack: Vec<CoordTile> = Vec::new();
    let mut tiles_to_claim: Vec<CoordTile> = vec![];

    stack.push(start_tile.coord_tile.clone());

    while let Some(curr_coord) = stack.pop() {
        if !visited.insert(curr_coord.clone()) {
            continue;
        }
        if let Some(ref_tile) = tiles_map.get_mut(&curr_coord) {
            if ref_tile.occupied == Occupied::Enemy {
                tiles_to_claim = vec![];
                break
            }
            if ref_tile.occupied == Occupied::Tail || ref_tile.occupied == Occupied::Player {
                continue;
            }
            if ref_tile.role == Role::Border {
                tiles_to_claim.push(ref_tile.coord_tile.clone());
                continue
            }
            if ref_tile.role == Role::Claimed { continue }
            tiles_to_claim.push(ref_tile.coord_tile.clone());
        }
        let directions = [
            (-1, 0), (1, 0),
            (0, -1), (0, 1),  
        ];
        for (dx, dy) in directions {
            let new_x = curr_coord.x as i64 + dx;
            let new_y = curr_coord.y as i64 + dy;

            if new_x >= 0 && new_x < (game.tile_dim.x - 1) as i64
                && new_y >= 0 && new_y < (game.tile_dim.y - 1) as i64
            {
                let new_coord = CoordTile {
                    x: new_x as u64,
                    y: new_y as u64,
                };

                if !visited.contains(&new_coord) {
                    stack.push(new_coord);
                }
            }
        }
    }
    tiles_to_claim
}
