use crate::transform::rotate_about_z;
use embedded_graphics::{
    pixelcolor::Rgb565,
    prelude::*,
    primitives::{Line, PrimitiveStyle, Rectangle, Triangle},
};
use embedded_graphics_simulator::{
    OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window,
};

use crate::DISP_SIZE;
use libm::{cos, sin};
pub fn generate_initial_grid() -> Vec<Vec<f32>> {
    let mut grid: Vec<Vec<f32>> = Vec::new();
    let spacing = 3;

    for i in 0..11 {
        for j in 0..11 {
            grid.push(vec![(i * spacing) as f32, (j * spacing) as f32, 0.0]);
        }
    }
    grid
}

pub fn update_x_y_axis(
    grid: &Vec<Vec<f32>>,
    rot: f64,
    phi_y: f64,
    display: &mut SimulatorDisplay<Rgb565>,
) -> Result<(), std::convert::Infallible> {
    let center = rotate_about_z(phi_y, vec![15.0, 15.0, 0.0]);
    let cpx = center[0];
    let cpy = center[1];
    let cpz = 0.0;
    let cq_x = (cpx - cpy) * cos(rot.to_radians()) as f32 * 15.0;
    let cq_y = ((cpx + cpy) * sin(rot.to_radians()) as f32 - cpz) * 15.0;
    let y_offset_grid = DISP_SIZE as i32 / 2 - cq_y as i32;
    let x_offset_grid = DISP_SIZE as i32 / 2 - cq_x as i32;
    let mut grid_points: Vec<Point> = Vec::new();
    for i in 0..grid.len() {
        let ps_rot_z = rotate_about_z(phi_y, vec![grid[i][0], grid[i][1], 0.0]);

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
        .draw(display)?;
        grid_points.push(Point::new(q_x + x_offset_grid, q_y + y_offset_grid));
    }

    for i in 0..11 {
        for j in 0..11 {
            let idx = i * 11 + j;
            if i + 1 < 11 {
                let idx_right = (i + 1) * 11 + j;
                Line::new(grid_points[idx], grid_points[idx_right])
                    .into_styled(PrimitiveStyle::with_stroke(Rgb565::new(255, 255, 255), 1))
                    .draw(display)?;
            }

            if j + 1 < 11 {
                let idx_down = idx + 1;
                Line::new(grid_points[idx], grid_points[idx_down])
                    .into_styled(PrimitiveStyle::with_stroke(Rgb565::new(255, 255, 255), 1))
                    .draw(display)?;
            }
        }
    }
    Ok(())
}
