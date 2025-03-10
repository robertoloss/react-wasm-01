use std::collections::HashMap;
use crate::xonix::{game::types::{CoordTile, Tile}, utils::log_out};
use super::types_sphere::Sphere;


pub fn move_spheres(
    spheres: &mut Vec<Sphere>,
    tiles_map: &mut HashMap<CoordTile, Tile>,
) {
    spheres
        .iter_mut()
        .for_each(|sphere| {
            sphere.moves();
            if sphere.check_collision(tiles_map) {
                sphere.moves();
            }
            sphere.occupy_tile(tiles_map);
        });
}
