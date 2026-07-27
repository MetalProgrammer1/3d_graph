use crate::points::Point3;
use libm::{cos, sin};
pub fn rotate_about_z(phi_y: f64, points: Point3) -> Point3 {
    let px_og = points.x;
    let py_og = points.y;
    let pz_og = points.z;
    let px = px_og * cos(phi_y.to_radians()) as f32 - py_og * sin(phi_y.to_radians()) as f32;
    let py = px_og * sin(phi_y.to_radians()) as f32 + py_og * cos(phi_y.to_radians()) as f32;
    let pz = pz_og;
    Point3 {
        x: px,
        y: py,
        z: pz,
    }
}

pub fn rotate_about_x(phi_y: f64, points: Point3) -> Point3 {
    let px_og = points.x;
    let py_og = points.y;
    let pz_og = points.z;

    let px = px_og;
    let py = py_og * cos(phi_y.to_radians()) as f32 - pz_og * sin(phi_y.to_radians()) as f32;
    let pz = py_og * sin(phi_y.to_radians()) as f32 + pz_og * cos(phi_y.to_radians()) as f32;

    Point3 {
        x: px,
        y: py,
        z: pz,
    }
}
