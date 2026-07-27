pub mod grid;
pub mod parser;
pub mod points;
pub mod transform;
use embedded_graphics::{
    mono_font::{MonoTextStyle, ascii::FONT_5X8},
    pixelcolor::Rgb565,
    prelude::*,
    primitives::{PrimitiveStyle, Rectangle},
    text::Text,
};
use embedded_graphics_simulator::{
    OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window,
};
use std::time::Instant;
use std::{thread, time::Duration};

use crate::grid::{generate_initial_grid, update_x_y_axis};
use crate::points::{display_points, generate_points, generate_screen_qs};

const DISP_SIZE: u32 = 500;
fn main() -> Result<(), std::convert::Infallible> {
    let mut display: SimulatorDisplay<Rgb565> =
        SimulatorDisplay::new(Size::new(DISP_SIZE, DISP_SIZE));
    let output_settings = OutputSettingsBuilder::new().scale(2).build();
    let mut window = Window::new("Graph", &output_settings);

    let ps: Vec<Vec<f32>> = generate_points();
    let style = MonoTextStyle::new(&FONT_5X8, Rgb565::WHITE);
    let grid: Vec<Vec<f32>> = generate_initial_grid();
    let rot: f64 = 30.0;
    let mut phi_y: f64 = 0.0;
    'running: loop {
        let _ = display.clear(Rgb565::new(1, 1, 1));
        let compute_start = Instant::now();
        update_x_y_axis(&grid, rot, phi_y, &mut display);

        let generated_screen_qs = generate_screen_qs(&ps, rot, phi_y);
        let mut qs: Vec<Vec<f32>> = generated_screen_qs.0;
        let y_offset = generated_screen_qs.1;
        phi_y += 5.0;
        display_points(&mut qs, y_offset, &mut display);
        qs.clear();
        let compute_time = compute_start.elapsed();
        println!("{:?}", compute_time);
        window.update(&display);
        for e in window.events() {
            match e {
                SimulatorEvent::Quit => {
                    break 'running Ok(());
                }
                _ => {}
            }
        }
        thread::sleep(Duration::from_millis(1));
    }
}
