use std::fmt::{self, Display};


pub struct Game {
    pub tile_dim: CoordTile,
    pub abs_dim: CoordAbs
}
impl Game {
    pub fn new(dims: CoordTile) -> Self {
        Self {
            tile_dim: dims.clone(),
            abs_dim: CoordAbs {
                x: (dims.x as f64) * Tile::get_size(),
                y: (dims.y as f64) * Tile::get_size()
            }
        }
    }
}

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub struct CoordTile {
    pub x: u64,
    pub y: u64
}
impl Display for CoordTile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive(Debug,Clone,PartialEq)]
pub struct CoordAbs {
    pub x: f64,
    pub y: f64
}
#[derive(Clone,Debug)]
pub struct Tile {
    pub coord_tile: CoordTile,
    pub coord_abs: CoordAbs,
    pub role: Role,
    pub occupied: Occupied,
}
impl Tile {
   pub fn get_size() -> f64 { 8. } // 8. set tile width 
}

#[derive(Debug,Clone,PartialEq)]
pub enum Role {
    Border,
    Board,
    Claimed,
}

#[derive(Debug,Clone,PartialEq)]
pub enum Occupied {
    Player,
    Enemy,
    Tail,
    Empty
}


