pub mod traits;

use traits::*;
use crate::engine::vector2f::Vec2f;
use crate::engine::{pixel::Pixel, MyEngine};

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
    fn add_to_frame(&self, screen: &mut MyEngine) {}
}

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


// TODO: make model as set of pixels
pub struct Model {
    model: Pixel,
}
impl Model {
    pub fn from_pixel(pixel: Pixel) -> Self {
        Self {
            model: pixel
        }
    }
}

enum Facing {
    Left,
    Right,
}

