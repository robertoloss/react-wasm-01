use crate::game::types::Vec2;


pub struct Player {
    pub pos: Vec2,
    //pub init_pos: Vec2,
    pub size: f64,
    pub delta: Vec2,
    pub angle: f64
}

impl Player {
    pub fn new() -> Self {
        let initial_position = Vec2 { x: 300., y: 300. };
        let angle_init: f64 = 0.;
        let delta_x = angle_init.cos() * 50.;
        let delta_y = angle_init.sin() * 50.;
        Player { 
            pos: initial_position.clone(), 
            //init_pos: initial_position,
            size: 8.,
            delta: Vec2 { x: delta_x, y: delta_y },
            angle: angle_init,
        }
    }
    pub fn moves_around(&mut self) {
        todo!()
    }
}
