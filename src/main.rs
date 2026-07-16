use embedded_graphics::{
    pixelcolor::Rgb565,
    prelude::*,
    primitives::{PrimitiveStyle, Rectangle, StyledDrawable, Triangle},
};
//use embedded_graphics_simulator::BinaryColorTheme;
use embedded_graphics_simulator::{
    OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window,
};

struct Q {
    q_x: i32,
    q_y: i32,
    minus_d: i32,
}

const DISP_SIZE: u32 = 400;
fn main() -> Result<(), std::convert::Infallible> {
    let mut display: SimulatorDisplay<Rgb565> =
        SimulatorDisplay::new(Size::new(DISP_SIZE, DISP_SIZE));
    let output_settings = OutputSettingsBuilder::new().scale(2).build();
    let mut window = Window::new("Graph", &output_settings);
    let stroke = PrimitiveStyle::with_stroke(Rgb565::WHITE, 3);
    let mid_point = DISP_SIZE as i32 / 2;

    let Ps: Vec<Vec<i32>> = vec![
        vec![0, 0, 0],
        vec![0, 1, 0],
        vec![1, 0, 0],
        vec![1, 1, 0],
        vec![0, 0, 1],
        vec![0, 1, 1],
        vec![1, 0, 1],
        vec![1, 1, 1],
    ];

    'running: loop {
        let mut Qs: Vec<Vec<i32>> = Vec::new();
        let minus_d = -1.0;
        let z_offset = -5.0;
        for i in Ps.iter() {
            let px = i[0] as f32;
            let py = i[1] as f32;
            let pz = i[2] as f32 + z_offset;
            let q_x = (600.0 * (minus_d * px) / pz) as i32 + 50;
            let q_y = (600.0 * (minus_d * py) / pz) as i32 + 50;
            Qs.push(vec![q_x, q_y]);
        }

        let _ = display.clear(Rgb565::new(1, 1, 1));

        for q in Qs.iter() {
            Rectangle::new(Point::new(q[0], q[1]), Size::new(3, 3))
                .into_styled(stroke)
                .draw(&mut display)?;
        }

        window.update(&display);

        for e in window.events() {
            match e {
                SimulatorEvent::Quit => {
                    break 'running Ok(());
                }
                _ => {}
            }
        }
    }
}
