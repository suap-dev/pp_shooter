use console_engine::Color;
use super::vector2f::Vec2f;

pub struct Pixel {
    pub color: Color,
    pub offset: Vec2f,
}
impl Pixel {
    pub fn from_color(color: Color) -> Self {
        Self { color, offset: Vec2f::zero() }
    }
}