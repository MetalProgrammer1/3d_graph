use crate::DrawItem;
use crate::points::Point3;
use crate::transform::rotate_about_z;
use crate::{DISP_SIZE, DrawPoint};
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::*;

use libm::{cos, sin};
pub fn generate_initial_grid() -> Vec<Point3> {
    let mut grid: Vec<Point3> = Vec::new();
    let spacing = 3;

    for i in 0..11 {
        for j in 0..11 {
            grid.push(Point3 {
                x: (i * spacing) as f32,
                y: (j * spacing) as f32,
                z: 0.0,
            });
        }
    }
    grid
}

pub fn send_to_display_grid(grid: &Vec<Point3>, rot: f64, phi_y: f64) -> Vec<DrawPoint> {
    let mut items: Vec<DrawPoint> = Vec::new();

    let center = rotate_about_z(
        phi_y,
        Point3 {
            x: 15.0,
            y: 15.0,
            z: 0.0,
        },
    );
    let cpx = center.x;
    let cpy = center.y;
    let cpz = 0.0;
    let cq_x = (cpx - cpy) * cos(rot.to_radians()) as f32 * 15.0;
    let cq_y = ((cpx + cpy) * sin(rot.to_radians()) as f32 - cpz) * 15.0;
    let y_offset_grid = DISP_SIZE as i32 / 2 - cq_y as i32;
    let x_offset_grid = DISP_SIZE as i32 / 2 - cq_x as i32;
    let mut grid_points: Vec<Point> = Vec::new();
    for i in 0..grid.len() {
        let ps_rot_z = rotate_about_z(
            phi_y,
            Point3 {
                x: grid[i].x,
                y: grid[i].y,
                z: 0.0,
            },
        );

        let px = ps_rot_z.x;
        let py = ps_rot_z.y;
        let pz = ps_rot_z.z;

        let q_x = ((px - py) * cos(rot.to_radians()) as f32 * 15.0) as i32;
        let q_y = (((px + py) * sin(rot.to_radians()) as f32 - pz) * 15.0) as i32;

        grid_points.push(Point::new(q_x + x_offset_grid, q_y + y_offset_grid));
    }

    for i in 0..11 {
        for j in 0..11 {
            let idx = i * 11 + j;
            if i + 1 < 11 {
                let idx_right = (i + 1) * 11 + j;
                items.push(DrawPoint {
                    item: DrawItem::Line {
                        a: grid_points[idx],
                        b: grid_points[idx_right],
                        color: Rgb565::new(255, 255, 255),
                    },
                    depth: 0.0,
                });
            }

            if j + 1 < 11 {
                let idx_down = idx + 1;

                items.push(DrawPoint {
                    item: DrawItem::Line {
                        a: grid_points[idx],
                        b: grid_points[idx_down],
                        color: Rgb565::new(255, 255, 255),
                    },
                    depth: 0.0,
                });
            }
        }
    }
    items
}
