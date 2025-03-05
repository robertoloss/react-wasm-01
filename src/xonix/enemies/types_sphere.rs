use crate::xonix::game::types::CoordAbs;

#[derive(Clone)]
enum UpDown {
    Up,
    Down
}
#[derive(Clone)]
enum LeftRight {
    Right,
    Left
}
struct Sphere {
    pub position: CoordAbs,
    pub dir_ud: UpDown,
    pub dir_lf: LeftRight
}
impl Sphere {
    pub fn moves(&self) {

    }
}
