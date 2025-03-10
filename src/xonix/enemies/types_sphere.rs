use std::{collections::HashMap, f32::consts::SQRT_2};
use crate::xonix::{game::types::{CoordAbs, CoordTile, Occupied, Role, Tile}, utils::log_out};

#[derive(Debug)]
pub struct Sphere {
    pub position: CoordAbs,
    pub velocity: CoordAbs,
    pub radius: f64,
    pub current_tile: Tile,
    pub previous_tile: Tile
}
impl Sphere {
    pub fn moves(&mut self) {
        self.position.x += self.velocity.x;
        self.position.y += self.velocity.y;
    }

    pub fn check_collision(
        &mut self, 
        tiles_map: &HashMap<CoordTile, Tile>,
    )
        -> bool
    {
        let rad = self.radius;
        let mut there_was_a_collision = false;
        let directions: Vec<((f64,f64),(f64,f64))> = [
            ((rad, 0.), (-1.,1.)),
            ((-rad, 0.), (-1.,1.)),
            ((0., rad), (1.,-1.)),
            ((0., -rad), (1.,-1.)),
        ].to_vec();
        for dir in directions {
            if sphere_is_colliding(
                CoordAbs {
                    x: self.position.x + dir.0.0,
                    y: self.position.y + dir.0.1,
                }, 
                tiles_map
            ) {
                self.velocity.x *= dir.1.0;
                self.velocity.y *= dir.1.1;
                //log_out(self.velocity.clone());
                there_was_a_collision = true
            }
        }
        there_was_a_collision
    }
    pub fn occupy_tile(
        &mut self,
        tiles_map: &mut HashMap<CoordTile, Tile>,
    ) {
        let coords = CoordTile {
            x: (self.position.x / Tile::get_size()) as u64,
            y: (self.position.y / Tile::get_size()) as u64
        };
        let tile = tiles_map
            .get_mut(&coords)
            .expect("No tile found in occupy_tile");

        if tile.coord_tile != self.current_tile.coord_tile {
            tile.occupied = Occupied::Enemy;
            self.previous_tile = self.current_tile.clone();
            self.current_tile = tile.clone();
        }
        tiles_map
            .get_mut(&self.previous_tile.coord_tile)
            .expect("No previous_tile found in occupy_tile")
            .occupied = Occupied::Empty;
    }
}

fn sphere_is_colliding(
    coord: CoordAbs,
    tiles_map: &HashMap<CoordTile, Tile>,
)
    -> bool
{
    let new_x = (coord.x / Tile::get_size()) as u64 + 1;
    let new_y = (coord.y / Tile::get_size()) as u64 + 1;

    let tile_size = Tile::get_size();

    if coord.x < tile_size || coord.y < tile_size {
        return true
    }

    if let Some(tile) = tiles_map
        .get(&CoordTile { 
            x: new_x,
            y: new_y
        }) {
        return tile.role == Role::Claimed
    } else {
        return true
    }
}




