use std::usize;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};
use crate::{map::get_map::get_map, player::types::Player, utils::main::log_out};


pub fn draw(
    ctx: &CanvasRenderingContext2d,
    canvas: &HtmlCanvasElement,
    player: &mut Player
) {
    ctx.set_fill_style_str("gray");
    ctx.fill_rect(0., 0., canvas.width() as f64, canvas.height() as f64);

    let map = get_map();

    for y in 0..map.len() {
        for x in 0..map[0].len() {
            let color = if map[y][x] == 1 { 
                "white"
            } else {
                "black"
            };
            draw_tile(x, y, ctx, color);
        }
    }

    ctx.set_fill_style_str("yellow");
    ctx.fill_rect(
        player.pos.x, 
        player.pos.y, 
        player.size, 
        player.size,
    );
    
    ctx.begin_path();
    ctx.set_line_width(3.0);
    ctx.set_stroke_style_str("red");
    ctx.move_to(
        player.pos.x + (player.size / 2.), 
        player.pos.y + (player.size / 2.)
    );
    ctx.line_to(
        player.pos.x + (player.size / 2.) + player.delta.x,
        player.pos.y + (player.size / 2.) + player.delta.y,
    );
    ctx.stroke();
    ctx.close_path();
}



fn draw_tile(
    x: usize, 
    y: usize, 
    ctx: &CanvasRenderingContext2d,
    color: &str
) {
    let tile_size = 64.;
    ctx.set_fill_style_str(color);
    ctx.fill_rect(
        ((x as f64) * tile_size) + 1., 
        ((y as f64) * tile_size) + 1., 
        tile_size - 2., 
        tile_size - 2.
    );
}
