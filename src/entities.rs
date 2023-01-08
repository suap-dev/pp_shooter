use console_engine::ConsoleEngine;

use crate::data_structs::{Direction, Facing, Vec2};

pub struct Projectile {
    pub model: [String; 2],
    pub facing: Facing,
    pub position: Vec2,
    pub velocity: Vec2,
    pub base_speed: f32,
    pub range: f32,
}

impl Projectile {
    pub fn new(model: [String; 2], base_speed: f32, range: f32) -> Projectile {
        Projectile {
            model,
            facing: Facing::Right,
            position: Vec2::zero(),
            velocity: Vec2::zero(),
            base_speed,
            range,
        }
    }

    pub fn clone_facing_at(&self, facing: Facing, position: Vec2) -> Projectile {
        Projectile {
            model: [self.model[0].clone(), self.model[1].clone()],
            facing,
            position,
            velocity: match facing {
                Facing::Right => Vec2 {
                    x: self.base_speed,
                    y: 0.0,
                },
                Facing::Left => Vec2 {
                    x: -self.base_speed,
                    y: 0.0,
                },
            },
            base_speed: self.base_speed,
            range: self.range,
        }
    }

    pub fn update_frame(&mut self, screen: &mut ConsoleEngine) {
        self.proceed_in_time();
        screen.print(
            self.position.x as i32,
            self.position.y as i32,
            self.model[self.facing as usize].as_str(),
        );
    }

    pub fn proceed_in_time(&mut self) {
        self.position += self.velocity;
        self.range -= self.velocity.magnitude();
    }

    pub fn remaining_range(&self) -> f32 {
        self.range
    }
}

pub struct ProjectileHandler {
    projectiles: Vec<Projectile>,
}
impl ProjectileHandler {
    pub fn new() -> ProjectileHandler {
        ProjectileHandler {
            projectiles: Vec::new(),
        }
    }

    pub fn handle(&mut self, projectile: Projectile) {
        self.projectiles.push(projectile);
    }

    pub fn update_frame(&mut self, screen: &mut ConsoleEngine) {
        self.projectiles.retain_mut(|projectile| {
            if projectile.remaining_range() > 0.0 {
                projectile.update_frame(screen);
                true
            } else {
                false
            }
        });
    }
}

pub struct Player {
    pub model: [String; 2],
    pub facing: Facing,
    pub position: Vec2,
    pub velocity: Vec2,
    pub base_speed: f32,
    pub projectile_type: Projectile,
}
impl Player {
    pub fn go(&mut self, direction: Direction) {
        self.velocity = match direction {
            Direction::Up => Vec2::new(0.0, -self.base_speed),
            Direction::Down => Vec2::new(0.0, self.base_speed),
            Direction::Left => {
                self.facing = Facing::Left;
                Vec2::new(-3.0 * self.base_speed, 0.0)
            }
            Direction::Right => {
                self.facing = Facing::Right;
                Vec2::new(3.0 * self.base_speed, 0.0)
            }
        }
    }

    pub fn stop(&mut self) {
        self.velocity = Vec2::zero();
    }

    pub fn proceed_in_time(&mut self) {
        self.position += self.velocity;
    }

    // pub fn shoot(&mut self, projectile: &mut Projectile) {
    //     // FIXME: generalise projectile update
    //     projectile.position = self.get_barrel_exit_coords();
    //     projectile.shoot(self.facing);
    // }

    pub fn shoot(&self) -> Projectile {
        self.projectile_type
            .clone_facing_at(self.facing, self.get_barrel_exit_coords())
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
