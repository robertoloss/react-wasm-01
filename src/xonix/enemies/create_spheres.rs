use crate::xonix::game::types::CoordAbs;
use super::types_sphere::Sphere;

pub fn create_spheres(spheres: &mut Vec<Sphere>) {
    let velocity = 3.;
    let sphere = Sphere {
        position: CoordAbs {
            x: 20.,
            y: 20.
        },
        radius: 8.,
        velocity: CoordAbs {
            x: velocity,
            y: velocity,
        },
    };
    spheres.push(sphere);
}
