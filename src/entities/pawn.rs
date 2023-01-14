use super::{actor::*, model::*, *};
use crate::engine::{vector2f::*, *};

pub struct Pawn {
    actor: Actor,
    velocity: Vec2f,
    screen_dimensions: Vec2f,
}
impl Pawn {
    pub fn new(screen: &MyEngine, model: Model, velocity: Vec2f) -> Self {
        Pawn {
            actor: Actor::from_model(model),
            velocity,
            screen_dimensions: Vec2f::inew(screen.get_width() as i32, screen.get_height() as i32),
        }
    }

    // pub fn update_position(&mut self) {
    //     self.actor.position += self.velocity;
    // }
    // pub fn update_velocity(&mut self) {
    //     if self.actor.position.x() <= 0.0
    //         || self.actor.position.x() >= self.screen_dimensions.x() - 2.0
    //     {
    //         self.velocity = Vec2f::new(-self.velocity.x(), self.velocity.y());
    //     }

    //     if self.actor.position.y() <= 0.0
    //         || self.actor.position.y() >= self.screen_dimensions.y() - 1.0
    //     {
    //         self.velocity = Vec2f::new(self.velocity.x(), -self.velocity.y());
    //     }
    // }
    // pub fn update(&mut self) {
    //     self.update_position();
    //     self.update_velocity();
    // }
}
impl Drawable for Pawn {
    fn add_to_frame(&self, screen: &mut MyEngine) {
        self.actor.add_to_frame(screen);
    }
}
impl Entity for Pawn {
    fn get_position(&self) -> Vec2f {
        self.actor.get_position()
    }
    fn set_position(&mut self, position: Vec2f) {
        self.actor.set_position(position);
    }
    fn translate(&mut self, vector: Vec2f) -> Vec2f {
        self.actor.translate(vector)
    }
}
