use std::{thread, time::Duration};

use embedded_graphics::{
    pixelcolor::{BinaryColor, Rgb565},
    prelude::*,
    primitives::{PrimitiveStyle, Triangle},
};
//use embedded_graphics_simulator::BinaryColorTheme;
use embedded_graphics_simulator::{
    OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window,
};
use libm::{atan, cos, sin};

const DISP_SIZE: u32 = 400;
fn main() -> Result<(), std::convert::Infallible> {
    let mut display: SimulatorDisplay<Rgb565> =
        SimulatorDisplay::new(Size::new(DISP_SIZE, DISP_SIZE));
    let output_settings = OutputSettingsBuilder::new().scale(2).build();
    let mut window = Window::new("Graph", &output_settings);
    let z_offset = -5.0;
    let mut x_offset = 0.0;
    let mut y_offset = 0.0;
    let scale = 50.0;
    let ps: Vec<Vec<f32>> = vec![
        vec![1.0, 0.0, 0.0],
        vec![0.0, 0.0, 0.0],
        vec![0.0, 1.0, 0.0],
        vec![1.0, 1.0, 0.0],
        vec![0.0, 0.0, 1.0],
        vec![0.0, 1.0, 1.0],
        vec![1.0, 0.0, 1.0],
        vec![1.0, 1.0, 1.0],
    ];

    let mut Qs: Vec<Vec<i32>> = Vec::new();

    //let vert_points: Vec<Vec<f32>> = Vec::new();

    let mut rot: f64 = 30.0;
    for i in ps.iter() {
        let px = i[0] as f32;
        let py = i[1] as f32;
        let pz = i[2] as f32 + z_offset;

        if px == 1.0 && py == 0.0 && pz + 5.0 == 0.0 {
            let q_x = (scale * (px - pz) * cos((rot).to_radians()) as f32) as i32;
            let q_y = (scale * ((px + pz) * sin((rot).to_radians()) as f32 - py)) as i32;

            println!("q_x {} q_y {}", q_x, q_y);
            x_offset = (DISP_SIZE as i32 / 2 - q_x) as f32;
            y_offset = (DISP_SIZE as i32 / 2 - q_y) as f32;
            println!("x_offset {} y_offset {}", x_offset, y_offset);
        }
        let q_x = (scale * (px - pz) * cos((rot).to_radians()) as f32 + x_offset) as i32;
        let q_y = (scale * ((px + pz) * sin((rot).to_radians()) as f32 - py) + y_offset) as i32;
        let q_z = pz as i32;
        println!("q_x {} q_y {} q_z {}", q_x, q_y, q_z);

        Qs.push(vec![q_x, q_y]);
    }
    'running: loop {
        let _ = display.clear(Rgb565::new(1, 1, 1));

        for i in Qs.iter() {
            Pixel(Point::new(i[0], i[1]), Rgb565::WHITE).draw(&mut display)?;
            //Triangle::new(Point::new(i[0], i[1]),)
        }

        //Qs.clear();
        window.update(&display);
        for e in window.events() {
            match e {
                SimulatorEvent::Quit => {
                    break 'running Ok(());
                }
                _ => {}
            }
        }
        thread::sleep(Duration::from_millis(500));
    }
}
