pub mod transform;
use embedded_graphics::{
    pixelcolor::{BinaryColor, Rgb565},
    prelude::*,
    primitives::{PrimitiveStyle, Rectangle, Triangle},
};
use std::{thread, time::Duration, vec};
//use embedded_graphics_simulator::BinaryColorTheme;
use embedded_graphics_simulator::{
    OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window,
};
use libm::{atan, cos, sin, sinh};

use crate::transform::rotate_about_y;

const DISP_SIZE: u32 = 500;
fn main() -> Result<(), std::convert::Infallible> {
    let mut display: SimulatorDisplay<Rgb565> =
        SimulatorDisplay::new(Size::new(DISP_SIZE, DISP_SIZE));
    let output_settings = OutputSettingsBuilder::new().scale(2).build();
    let mut window = Window::new("Graph", &output_settings);
    let z_offset = -0.0;
    let mut x_offset = 0;
    let mut y_offset = 0;
    let scale = 1.0;
    let mut ps: Vec<Vec<f32>> = Vec::new();
    let mut qs: Vec<Vec<f32>> = Vec::new();

    //let vert_points: Vec<Vec<f32>> = Vec::new();
    let spacing = 1;

    for i in -100..100 {
        for j in -100..100 {
            let x = i as f32 * 0.03;
            let y = j as f32 * 0.03;

            let z = x.powf(2.0) + y.powf(2.0);
            let z1 = sin(x.powf(2.0) as f64) + cos(y.powf(2.0) as f64) as f64;
            //let z = (-x.powf(2.0) - y.powf(2.0) + 20.0).powf(0.5);
            if !z.is_nan() {
                //ps.push(vec![x, y, z as f32]);
                //ps.push(vec![x, y, z1 as f32]);
                ps.push(vec![(i * spacing) as f32, (j * spacing) as f32, 0.0]);
                //ps.push(vec![x, y, -z]);
            }
        }
    }

    let rot: f64 = 30.0;
    let mut phi_y: f64 = 0.0;
    //let  phi_x: f64 = 0.0;
    'running: loop {
        let _ = display.clear(Rgb565::new(1, 1, 1));

        for i in ps.iter() {
            let px_og = i[0] as f32;
            let py_og = i[1] as f32;
            let pz_og = i[2] as f32 + z_offset;

            // let px1 =
            //     px_og * cos(phi_y.to_radians()) as f32 - py_og * sin(phi_y.to_radians()) as f32;
            // let py1 =
            //     px_og * sin(phi_y.to_radians()) as f32 + py_og * cos(phi_y.to_radians()) as f32;
            // let pz1 = pz_og;

            let ps_rotated = rotate_about_y(vec![px_og, py_og, pz_og]);

            let px = px1;
            let py = py1 * cos(phi_y.to_radians()) as f32 - pz1 * sin(phi_y.to_radians()) as f32;
            let pz = py1 * sin(phi_y.to_radians()) as f32 + pz1 * cos(phi_y.to_radians()) as f32;

            //py = (cos(phi.to_radians()) as f32 * py - sin(phi.to_radians()) as f32 * pz) as f32;
            //pz = (sin(phi.to_radians()) as f32 * py + cos(phi.to_radians()) as f32 * pz) as f32;

            let q_x = ((px - py) * cos(rot.to_radians()) as f32 * 15.0) + 250.0;
            let q_y = (((px + py) * sin(rot.to_radians()) as f32 - pz) * 15.0);
            let q_z = pz;

            //println!("q_x {} q_y {} q_z {}", q_x, q_y, q_z);
            if px_og == 0.0 && py_og == 0.0 && py_og == 0.0 {
                y_offset = DISP_SIZE as i32 / 2 - q_y as i32;
            }
            qs.push(vec![q_x, q_y, q_z]);
        }
        //let max: Vec<&Vec<i32>> = 0;
        //println!("max {:?}", max);
        //let y_offset = 0; //350 - max;
        phi_y += 5.0;
        // if phi_y >= 1000.0 {
        //     phi_y = 0.0;
        // }

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
            .draw(&mut display)?;
        }

        qs.clear();
        window.update(&display);
        for e in window.events() {
            match e {
                SimulatorEvent::Quit => {
                    break 'running Ok(());
                }
                _ => {}
            }
        }
        thread::sleep(Duration::from_millis(50));
    }
}
