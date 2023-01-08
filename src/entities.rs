use console_engine::ConsoleEngine;

use crate::data_structs::{Direction, Facing, Vec2};

pub struct Projectile {
    pub model: [String; 2],
    pub facing: Facing,
    pub position: Vec2,
    pub velocity: Vec2,
    pub base_speed: f32,
}

impl Projectile {
    pub fn update_frame(&mut self, screen: &mut ConsoleEngine) {
        self.proceed_in_time();
        screen.print(
            self.position.x as i32,
            self.position.y as i32,
            self.model[self.facing as usize].as_str(),
        );
    }

    pub fn shoot(&mut self, facing: Facing) {
        self.velocity = match facing {
            Facing::Right => Vec2::new(self.base_speed, 0.0),
            Facing::Left => Vec2::new(-self.base_speed, 0.0),
        };        
        self.facing = facing
    }

    pub fn proceed_in_time(&mut self) {
        self.position += self.velocity;
    }
}

pub struct Player {
    pub model: [String; 2],
    pub facing: Facing,
    pub position: Vec2,
    pub velocity: Vec2,
    pub base_speed: f32,
}
impl Player {
    pub fn go(&mut self, direction: Direction) {
        self.velocity = match direction {
            Direction::Up => Vec2::new(0.0, -self.base_speed),
            Direction::Down => Vec2::new(0.0, self.base_speed),
            Direction::Left => {
                self.facing = Facing::Left;
                Vec2::new(-2.0 * self.base_speed, 0.0)
            }
            Direction::Right => {
                self.facing = Facing::Right;
                Vec2::new(2.0 * self.base_speed, 0.0)
            }
        }
    }

    pub fn stop(&mut self) {
        self.velocity = Vec2::zero();
    }

    pub fn proceed_in_time(&mut self) {
        self.position += self.velocity;
    }

    pub fn shoot(&mut self, projectile: &mut Projectile) {
        // FIXME: generalise projectile update
        projectile.position = self.get_barrel_exit_coords();
        projectile.shoot(self.facing);
    }

    pub fn get_barrel_exit_coords(&self) -> Vec2 {
        // this is very dirty
        // TODO: make it better...
        let mut len = 0;
        for _ in self.model[1].chars() {
            len += 1;
        }

        // this is very... situational...
        let x: f32 = match self.facing {
            Facing::Right => self.position.x + (len as f32),
            Facing::Left => self.position.x - 1.,
        };

        let y: f32 = self.position.y;

        Vec2 { x, y }
    }

    pub fn update_frame(&mut self, screen: &mut ConsoleEngine) {
        self.proceed_in_time();
        screen.print(
            self.position.x as i32,
            self.position.y as i32,
            self.model[self.facing as usize].as_str(),
        );
    }
}
