use std::collections::HashMap;
use web_sys::CanvasRenderingContext2d;
use crate::enemies::types_sphere::Sphere;
use super::types::{CoordTile, Occupied, Role, Tile};


pub fn draw(
    tiles_map: &mut HashMap<CoordTile, Tile>,
    ctx: &CanvasRenderingContext2d,
    spheres: &Vec<Sphere>
) {
    for (_, tile) in tiles_map.iter() {
        match tile.role {
            Role::Board =>  { ctx.set_fill_style_str("black") }
            Role::Claimed => { ctx.set_fill_style_str("transparent") }
            Role::Border => { ctx.set_fill_style_str("transparent");}
        }
        match tile.occupied {
            Occupied::Player => { ctx.set_fill_style_str("blue") }
            Occupied::Tail => { ctx.set_fill_style_str("orange") }
            Occupied::Enemy => {}
            Occupied::Empty => {}
        }
        ctx.fill_rect(
            tile.coord_abs.x, 
            tile.coord_abs.y, 
            Tile::get_size(), 
            Tile::get_size()
        );
    };
    for sphere in spheres {
        draw_sphere(
            sphere, 
            ctx
        );
    }
}

fn draw_sphere(
    sphere: &Sphere,
    ctx: &CanvasRenderingContext2d
) {
    ctx.begin_path();
    ctx.arc(
        sphere.position.x, 
        sphere.position.y, 
        sphere.radius, 
        0.0, 
        std::f64::consts::PI * 2.0
    ).unwrap();
    ctx.set_fill_style_str("red");
    ctx.fill();
    ctx.close_path();
}
