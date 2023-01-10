use console_engine::ConsoleEngine;

use crate::data_structs::{Coords, Facing};

#[derive(Clone)]
pub struct Projectile {
    pub position: Coords,
    pub model: String,
    pub velocity: i32,
    pub valid: bool,
}
impl Projectile {
    pub fn update_frame(&mut self, screen: &mut ConsoleEngine) {
        self.proceed_in_time(screen);
        screen.print(self.position.x, self.position.y, self.model.as_str());
    }

    pub fn proceed_in_time(&mut self, screen: &ConsoleEngine) {
        if self.position.x > screen.get_width() as i32 || self.position.x < 0 {
            self.valid = false
        } else {
            self.position.x += self.velocity;
        }
    }
}
pub struct Player {
    pub model: [String; 2],
    pub facing: Facing,
    pub position: Coords,
    pub should_shoot: bool,
    pub valid: bool,
}
impl Player {
    pub fn go_right(&mut self, screen: &ConsoleEngine) {
        let mut len = 0;
        for _ in self.model[1].chars() {
            len += 1;
        }

        if self.position.x > screen.get_width() as i32 - len - 1 {
            self.valid = false
        } else {
            self.position.x += 1;
            self.facing = Facing::Right;
        }
    }

    pub fn go_left(&mut self, screen: &ConsoleEngine) {
        if self.position.x <= 0 {
            self.valid = false
        } else {
            self.position.x -= 1;
            self.facing = Facing::Left;
        }
    }

    pub fn go_up(&mut self, screen: &ConsoleEngine) {
        if self.position.y <= 0 {
            self.valid = false
        } else {
            self.position.y -= 1;
        }
    }

    pub fn go_down(&mut self, screen: &ConsoleEngine) {
        if self.position.y >= screen.get_height() as i32 - 1 {
            self.valid = false
        } else {
            self.position.y += 1;
        }
    }

    pub fn shoot(&mut self, projectile: &Projectile) -> Projectile {
        let mut projectile_clone = projectile.clone();

        // FIXME: generalise projectile update
        projectile_clone.position = self.get_barrel_exit_coords();
        projectile_clone.velocity = match self.facing {
            Facing::Right => 1,
            Facing::Left => -1,
        };
        // returning the clone
        projectile_clone

        // lvalue vs rvalue
        // "-" ;
        // 09 ;
        // assert!("-3".to_string() == String::from("-3"));
    }

    pub fn get_barrel_exit_coords(&self) -> Coords {
        // this is very dirty
        // TODO: make it better...
        let mut len = 0;
        for _ in self.model[1].chars() {
            len += 1;
        }

        // this is very... situational...
        let x: i32 = match self.facing {
            Facing::Right => self.position.x + len as i32,
            Facing::Left => self.position.x - 1,
        };

        let y: i32 = self.position.y;

        Coords { x, y }
    }

    pub fn update_frame(&mut self, screen: &mut ConsoleEngine) {
        screen.print(
            self.position.x,
            self.position.y,
            self.model[self.facing as usize].as_str(),
        );
    }
}
