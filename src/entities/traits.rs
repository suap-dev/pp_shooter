use crate::engine::MyEngine;
use crate::engine::vector2f::Vec2f;

pub trait Entity {
    fn translate(&mut self, vector: Vec2f) -> Vec2f;
    fn set_position(&mut self, position: Vec2f);
    fn get_position(&self) -> Vec2f;
}

pub trait Drawable {
    fn add_to_frame(&self, screen: &mut MyEngine);
}