pub mod transform;
use embedded_graphics::{
    pixelcolor::Rgb565,
    prelude::*,
    primitives::{Line, PrimitiveStyle, Rectangle, Triangle},
};
use std::{thread, time::Duration, vec};

use embedded_graphics_simulator::{
    OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window,
};
use libm::{cos, sin};

use crate::transform::{rotate_about_x, rotate_about_z};

const DISP_SIZE: u32 = 500;
fn main() -> Result<(), std::convert::Infallible> {
    let mut display: SimulatorDisplay<Rgb565> =
        SimulatorDisplay::new(Size::new(DISP_SIZE, DISP_SIZE));
    let output_settings = OutputSettingsBuilder::new().scale(2).build();
    let mut window = Window::new("Graph", &output_settings);

    let mut y_offset = 0;

    let mut ps: Vec<Vec<f32>> = Vec::new();
    let mut qs: Vec<Vec<f32>> = Vec::new();

    let mut grid: Vec<Vec<f32>> = Vec::new();

    //let vert_points: Vec<Vec<f32>> = Vec::new();
    let spacing = 3;

    for i in -100..100 {
        for j in -100..100 {
            let x = i as f32 * 0.03;
            let y = j as f32 * 0.03;

            let z = sin(x.powf(2.0) as f64) + cos(y.powf(2.0) as f64) as f64;

            if !z.is_nan() {
                ps.push(vec![x, y, z as f32]);
            }
        }
    }
    for i in 0..11 {
        for j in 0..11 {
            grid.push(vec![(i * spacing) as f32, (j * spacing) as f32, 0.0]);
        }
    }
    let rot: f64 = 30.0;
    let mut phi_y: f64 = 0.0;
    'running: loop {
        let _ = display.clear(Rgb565::new(1, 1, 1));

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

        phi_y += 5.0;

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
        println!("{}", grid.len());

        let center = rotate_about_z(phi_y, vec![15.0, 15.0, 0.0]);
        let cpx = center[0];
        let cpy = center[1];
        let cpz = 0.0;
        let cq_x = ((cpx - cpy) * cos(rot.to_radians()) as f32 * 15.0);
        let cq_y = ((cpx + cpy) * sin(rot.to_radians()) as f32 - cpz) * 15.0;
        let y_offset_grid = DISP_SIZE as i32 / 2 - cq_y as i32;
        let x_offset_grid = DISP_SIZE as i32 / 2 - cq_x as i32;
        let mut grid_points: Vec<Point> = Vec::new();
        for i in 0..grid.len() {
            let ps_rot_z = rotate_about_z(phi_y, vec![grid[i][0], grid[i][1], 0.0]);
            //let ps_rot_x = rotate_about_x(phi_y, vec![ps_rot_z[0], ps_rot_z[1], ps_rot_z[2]]);
            let px = ps_rot_z[0];
            let py = ps_rot_z[1];
            let pz = ps_rot_z[2];

            let q_x = ((px - py) * cos(rot.to_radians()) as f32 * 15.0) as i32;
            let q_y = (((px + py) * sin(rot.to_radians()) as f32 - pz) * 15.0) as i32;
            let q_z = pz;

            Rectangle::new(
                Point::new(q_x + x_offset_grid, q_y + y_offset_grid),
                Size::new(2, 2),
            )
            .into_styled(PrimitiveStyle::with_fill(Rgb565::new(255, 255, 255)))
            .draw(&mut display)?;
            grid_points.push(Point::new(q_x + x_offset_grid, q_y + y_offset_grid));
        }

        for i in 0..11 {
            for j in 0..11 {
                let idx = i * 11 + j;
                if i + 1 < 11 {
                    let idx_right = (i + 1) * 11 + j;
                    Line::new(grid_points[idx], grid_points[idx_right])
                        .into_styled(PrimitiveStyle::with_stroke(Rgb565::new(255, 255, 255), 1))
                        .draw(&mut display)?;
                }

                if j + 1 < 11 {
                    let idx_down = idx + 1;
                    Line::new(grid_points[idx], grid_points[idx_down])
                        .into_styled(PrimitiveStyle::with_stroke(Rgb565::new(255, 255, 255), 1))
                        .draw(&mut display)?;
                }
            }
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
        thread::sleep(Duration::from_millis(40));
    }
}
