use std::collections::HashMap;
use crate::game::types::{CoordTile, Tile};
use super::types_sphere::Sphere;


pub fn move_spheres(
    spheres: &mut Vec<Sphere>,
    tiles_map: &mut HashMap<CoordTile, Tile>,
) {
    spheres
        .iter_mut()
        .for_each(|sphere| {
            sphere.check_collision(tiles_map);
            sphere.moves();
            sphere.occupy_tile(tiles_map);
        });
}
