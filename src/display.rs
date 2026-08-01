use crate::{DrawItem, DrawPoint};
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::{
    Drawable,
    geometry::Size,
    primitives::{Line, Primitive, PrimitiveStyle, Rectangle},
};
use embedded_graphics_simulator::SimulatorDisplay;

pub fn display_all(
    items: &mut Vec<DrawPoint>,
    display: &mut SimulatorDisplay<Rgb565>,
) -> Result<(), std::convert::Infallible> {
    items.sort_by(|a, b| {
        a.depth
            .partial_cmp(&b.depth)
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    for i in items.iter() {
        match &i.item {
            DrawItem::Rect { pos, size, color } => {
                Rectangle::new(*pos, Size::new(*size, *size))
                    .into_styled(PrimitiveStyle::with_fill(*color))
                    .draw(display)?;
            }
            DrawItem::Line { a, b } => {
                Line::new(*a, *b)
                    .into_styled(PrimitiveStyle::with_stroke(Rgb565::new(255, 255, 255), 1))
                    .draw(display)?;
            }
        }
    }
    Ok(())
}
