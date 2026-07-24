use libm::{cos, sin};

pub fn rotate_about_z(phi_y: f64, points: Vec<f32>) -> Vec<f32> {
    let px_og = points[0];
    let py_og = points[1];
    let pz_og = points[2];
    let px = px_og * cos(phi_y.to_radians()) as f32 - py_og * sin(phi_y.to_radians()) as f32;
    let py = px_og * sin(phi_y.to_radians()) as f32 + py_og * cos(phi_y.to_radians()) as f32;
    let pz = pz_og;
    vec![px, py, pz]
}

pub fn rotate_about_x(phi_y: f64, points: Vec<f32>) -> Vec<f32> {
    let px_og = points[0];
    let py_og = points[1];
    let pz_og = points[2];

    let px = px_og;
    let py = py_og * cos(phi_y.to_radians()) as f32 - pz_og * sin(phi_y.to_radians()) as f32;
    let pz = py_og * sin(phi_y.to_radians()) as f32 + pz_og * cos(phi_y.to_radians()) as f32;

    vec![px, py, pz]
}
