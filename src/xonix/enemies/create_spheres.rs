use std::collections::HashMap;
use crate::xonix::game::types::{CoordAbs, CoordTile, Tile};
use super::types_sphere::Sphere;

pub fn create_spheres(
    spheres: &mut Vec<Sphere>,
    tiles_map: &HashMap<CoordTile, Tile>,
) {
    let coord_x = 20.;
    let coord_y = 20.;

    let tile = tiles_map
        .get(&CoordTile { 
            x: (coord_x / Tile::get_size()) as u64,
            y: (coord_y / Tile::get_size()) as u64,
        })
        .expect("No tile found creating a sphere");

    let velocity = 3.8;

    let sphere = Sphere {
        position: CoordAbs {
            x: coord_x,
            y: coord_y
        },
        radius: 8.,
        velocity: CoordAbs {
            x: velocity,
            y: velocity,
        },
        current_tile: tile.clone(),
        previous_tile: tile.clone(),
    };
    spheres.push(sphere);
}
