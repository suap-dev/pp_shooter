use super::{model::*, *};
use crate::engine::{vector2f::*, *};

pub struct Actor {
    model: Model,
    facing: Facing,
    position: Vec2f,
}
impl Actor {
    pub fn from_model(model: Model) -> Self {
        Actor {
            model,
            facing: Facing::Right,
            position: Default::default(),
        }
    }
    fn set_facing(&mut self, facing: Facing) {
        self.facing = facing;
    }
}
impl Entity for Actor {
    fn translate(&mut self, vector: Vec2f) -> Vec2f {
        self.position += vector;
        self.position
    }
    fn set_position(&mut self, position: Vec2f) {
        self.position = position;
    }
    fn get_position(&self) -> Vec2f {
        self.position
    }
}
impl Drawable for Actor {
    fn add_to_screen(&self, screen: &mut MyEngine) {
        for px in &self.model.pixels {
            screen.set_pixel(self.position, px);
        }
    }
}
