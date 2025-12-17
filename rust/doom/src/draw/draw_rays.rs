use std::f64::consts::PI;
use web_sys::CanvasRenderingContext2d;
use crate::{map::get_map::get_map, player::types::Player};


pub fn draw_rays(
    player: &mut Player, 
    ctx: &CanvasRenderingContext2d) 
{
    let num_of_rays = 1;
    let ra = player.angle;
    let px = player.pos.x;
    let py = player.pos.y;
    let map = get_map();

    for _r in 0..num_of_rays {
        let mut dof = 0;
        let a_tan = -1.0 / ra.tan();
        let (mut rx, mut ry, xo, yo);

        if ra > PI {
            ry = ((py as i32 >> 6) << 6) as f64 - 0.0001;
            rx = (py - ry) * a_tan + px;
            yo = -64.0;
            xo = -yo * a_tan;
        } else if ra < PI {
            ry = ((py as i32 >> 6) << 6) as f64 + 64.0;
            rx = (py - ry) * a_tan + px;
            yo = 64.0;
            xo = -yo * a_tan;
        } else {
            rx = px;
            ry = py;
            dof = 8;
            xo = 0.0;
            yo = 0.0;
        }

        while dof < 8 {
            let mx = (rx as i32) >> 6;
            let my = (ry as i32) >> 6;

            if my >= 0 && my < map.len() as i32 && mx >= 0 && mx < map[0].len() as i32 {
                let mp = map[my as usize][mx as usize];
                if mp == 1 {
                    dof = 8;
                } else {
                    rx += xo;
                    ry += yo;
                    dof += 1;
                }
            } else {
                dof = 8;
            }
        }

        ctx.set_stroke_style_str("green");
        ctx.set_line_width(2.0);
        ctx.begin_path();
        ctx.move_to(
            px + (player.size / 2.) , 
            py + (player.size / 2.)
        );
        ctx.line_to(rx, ry);
        ctx.stroke();
        ctx.close_path();
    }
}
