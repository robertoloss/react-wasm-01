use std::collections::HashMap;
use web_sys::CanvasRenderingContext2d;
use super::types::{CoordTile, Occupied, Role, Tile};


pub fn draw(
    tiles_map: &mut HashMap<CoordTile, Tile>,
    ctx: &CanvasRenderingContext2d
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
            Occupied::Enemy => { ctx.set_fill_style_str("red") }
            Occupied::Empty => { }
        }
        ctx.fill_rect(
            tile.coord_abs.x, 
            tile.coord_abs.y, 
            Tile::get_size(), 
            Tile::get_size()
        );
    };
}
