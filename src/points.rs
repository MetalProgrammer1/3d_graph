use crate::DISP_SIZE;
use crate::DrawItem;
use crate::DrawPoint;
use crate::parser::{eval, parser};
use crate::transform::rotate_about_z;
use chumsky::prelude::*;
use embedded_graphics::{pixelcolor::Rgb565, prelude::*};
use libm::{cos, sin};

pub struct Point3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

pub fn generate_points() -> Vec<Vec<Point3>> {
    let mut diff_ps: Vec<Vec<Point3>> = Vec::new();
    let graphs: Vec<&str> = vec!["x^2+y^2", "cos(x^2)+y", "ln(y^2)", "x^3+y^3"];

    for src in graphs.iter() {
        let ast = parser().parse(&src).into_result().unwrap();

        let mut ps: Vec<Point3> = Vec::new();
        for i in -30..30 {
            for j in -30..30 {
                let x = i as f32 * 0.2;
                let y = j as f32 * 0.2;

                let z = eval(&ast, x, y);
                //println!("z:{}", z);
                if !z.is_nan() && z.is_finite() {
                    ps.push(Point3 { x: x, y: y, z: z });
                }
            }
        }
        diff_ps.push(ps);
    }
    diff_ps
}

pub fn generate_screen_qs(
    diff_ps: &Vec<Vec<Point3>>,
    rot: f64,
    phi_y: f64,
) -> (Vec<Vec<Point3>>, i32) {
    let mut diff_qs: Vec<Vec<Point3>> = Vec::new();
    let y_offset = DISP_SIZE as i32 / 2;
    let rot_cos = cos(rot.to_radians()) as f32 * 15.0;
    let rot_sin = sin(rot.to_radians()) as f32;
    for ps in diff_ps.iter() {
        let mut qs: Vec<Point3> = Vec::new();
        for i in ps.iter() {
            let px_og = i.x as f32;
            let py_og = i.y as f32;
            let pz_og = i.z as f32;

            let ps_rot_z = rotate_about_z(
                phi_y,
                Point3 {
                    x: px_og,
                    y: py_og,
                    z: pz_og,
                },
            );

            let px = ps_rot_z.x;
            let py = ps_rot_z.y;
            let pz = ps_rot_z.z;

            let q_x = ((px - py) * rot_cos) + 250.0;
            let q_y = ((px + py) * rot_sin - pz) * 15.0;
            let q_z = pz;

            qs.push(Point3 {
                x: q_x,
                y: q_y,
                z: q_z,
            });
        }
        diff_qs.push(qs);
    }
    (diff_qs, y_offset)
}

pub fn send_to_display_points(
    diff_qs: &mut Vec<Vec<Point3>>,
    y_offset: i32,
    colours: &Vec<Vec<f32>>,
) -> Vec<Vec<DrawPoint>> {
    let mut graph_all_items: Vec<Vec<DrawPoint>> = Vec::new();
    let mut count = 0;
    for qs in diff_qs.iter_mut() {
        let colour = &colours[count];
        qs.sort_by_key(|d| d.z.clone() as i32);
        let mut items: Vec<DrawPoint> = Vec::new();
        let z_min = qs.iter().min_by(|a, b| a.z.partial_cmp(&b.z).unwrap());
        let z_max = qs.iter().max_by(|a, b| a.z.partial_cmp(&b.z).unwrap());
        for i in qs.iter() {
            let t =
                ((i.z - z_min.unwrap().z) / (z_max.unwrap().z - z_min.unwrap().z)).clamp(0.0, 1.0);
            let brightness = 0.2 + 0.8 * t;

            let r = (colour[0] * brightness).clamp(0.0, 31.0) as u8;

            let g = (colour[1] * brightness).clamp(0.0, 63.0) as u8;
            let b = (colour[2] * brightness).clamp(0.0, 31.0) as u8;
            let dist_from_center = (i.x.powf(2.0) + i.y.powf(2.0)).powf(0.5);
            let rect_size = 1; //(0.0 + dist_from_center * 0.1).clamp(1.0, 30.0) as u32;
            let half = rect_size as i32 / 2;

            items.push(DrawPoint {
                item: DrawItem::Rect {
                    pos: Point::new(i.x as i32 - half, i.y as i32 + y_offset - half),
                    size: rect_size,
                    color: Rgb565::new(r, g, b),
                },
                depth: i.z,
            });
        }
        graph_all_items.push(items);
        count = count + 1;
    }

    graph_all_items
}
