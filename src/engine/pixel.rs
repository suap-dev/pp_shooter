use super::vector2f::Vec2f;
use console_engine::Color;

pub struct Pixel {
    pub color: Color,
    pub offset: Vec2f,
}
impl Pixel {
    pub fn new(color: Color, offset: Vec2f) -> Self {
        Self { color, offset }
    }

    pub fn from_color(color: Color) -> Self {
        Self {
            color,
            offset: Vec2f::zero(),
        }
    }
}
