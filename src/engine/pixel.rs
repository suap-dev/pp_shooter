use super::vector2f::Vec2f;
use console_engine::Color;

pub struct Pixel {
    pub ce_pix: console_engine::pixel::Pixel,
    pub color: Color,
    pub offset: Vec2f,
}
impl Pixel {
    pub fn new(color: Color, offset: Vec2f) -> Self {
        let ce_pix = console_engine::pixel::pxl_fbg(' ', color, color);
        Self { ce_pix, color, offset }
        
    }

    // pub fn from_color(color: Color) -> Self {
    //     Self {
    //         color,
    //         offset: Vec2f::zero(),
    //     }
    // }
}
