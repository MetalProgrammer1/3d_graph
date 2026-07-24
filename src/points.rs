use crate::DISP_SIZE;
use crate::transform::rotate_about_z;
use embedded_graphics::{
    pixelcolor::Rgb565,
    prelude::*,
    primitives::{Line, PrimitiveStyle, Rectangle, Triangle},
};
use embedded_graphics_simulator::SimulatorDisplay;
use libm::{cos, sin};

pub fn generate_points() -> Vec<Vec<f32>> {
    let mut ps: Vec<Vec<f32>> = Vec::new();
    for i in -100..100 {
        for j in -100..100 {
            let x = i as f32 * 0.07;
            let y = j as f32 * 0.07;

            let z = sin(x.powf(2.0) as f64) + cos(y.powf(2.0) as f64) as f64;

            if !z.is_nan() {
                ps.push(vec![x, y, z as f32]);
            }
        }
    }
    ps
}

pub fn generate_screen_qs(ps: &Vec<Vec<f32>>, rot: f64, phi_y: f64) -> (Vec<Vec<f32>>, i32) {
    let mut qs: Vec<Vec<f32>> = Vec::new();
    let mut y_offset = 0;
    for i in ps.iter() {
        let px_og = i[0] as f32;
        let py_og = i[1] as f32;
        let pz_og = i[2] as f32;

        let ps_rot_z = rotate_about_z(phi_y, vec![px_og, py_og, pz_og]);

        //let ps_rot_x = rotate_about_x(phi_y, vec![ps_rot_z[0], ps_rot_z[1], ps_rot_z[2]]);

        let px = ps_rot_z[0];
        let py = ps_rot_z[1];
        let pz = ps_rot_z[2];

        let q_x = ((px - py) * cos(rot.to_radians()) as f32 * 15.0) + 250.0;
        let q_y = ((px + py) * sin(rot.to_radians()) as f32 - pz) * 15.0;
        let q_z = pz;

        if px_og == 0.0 && py_og == 0.0 && py_og == 0.0 {
            y_offset = DISP_SIZE as i32 / 2 - q_y as i32;
        }
        qs.push(vec![q_x, q_y, q_z]);
    }
    (qs, y_offset)
}

pub fn display_points(
    qs: &mut Vec<Vec<f32>>,
    y_offset: i32,
    display: &mut SimulatorDisplay<Rgb565>,
) -> Result<(), std::convert::Infallible> {
    qs.sort_by(|a, b| a[2].partial_cmp(&b[2]).unwrap());
    for i in qs.iter() {
        let shade = (i[2].abs() as f32 * 5.0 + 0.01).min(30.0);
        let r = 31_u8.saturating_sub(shade as u8);
        let g = 63_u8.saturating_sub(shade as u8 * 2);
        let b = 31_u8.saturating_sub(shade as u8);

        Rectangle::new(
            Point::new(i[0] as i32, i[1] as i32 + y_offset),
            Size::new(3, 3),
        )
        .into_styled(PrimitiveStyle::with_fill(Rgb565::new(r, g, b)))
        .draw(display)?;
    }
    Ok(())
}
