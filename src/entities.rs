use console_engine::ConsoleEngine;

use crate::data_structs::{Coords, Facing};

#[derive(Clone)]
pub struct Projectile {
    pub position: Coords,
    pub model: String,
    pub velocity: i32,
}
impl Projectile {
    pub fn update_frame(&mut self, screen: &mut ConsoleEngine) {
        self.proceed_in_time();
        screen.print(self.position.x, self.position.y, self.model.as_str());
    }

    pub fn proceed_in_time(&mut self) {
        self.position.x += self.velocity;
    }


}
pub struct Player {
    pub model: [String; 2],
    pub facing: Facing,
    pub position: Coords,
    pub should_shoot: bool,
}
impl Player {
    pub fn go_right(&mut self) {
        self.position.x += 1;
        self.facing = Facing::Right;
    }

    pub fn go_left(&mut self) {
        self.position.x -= 1;
        self.facing = Facing::Left;
    }

    pub fn go_up(&mut self) {
        self.position.y -= 1;
    }

    pub fn go_down(&mut self) {
        self.position.y += 1;
    }

    pub fn shoot(&mut self, projectile: &Projectile) -> Projectile{
        let mut projectile_clone = projectile.clone();

        // FIXME: generalise projectile update
        projectile_clone.position = self.get_barrel_exit_coords();        
        projectile_clone.velocity = match self.facing {
            Facing::Right => 1,
            Facing::Left => -1,
        
        };
        // returning the clone
        projectile_clone

        // let mut bullet2 = projectile.clone();

        // return bullet2;

        // let mut bullet2: Projectile = Projectile {
        //     position: Coords { x: -1, y: -1 },
        //     model: String::from("-"),
        //     velocity: 1,
        // };
    

        // projectile.model = "-".to_string();

        // lvalue vs rvalue
        // "-" ;
        // 09 ;
        // assert!("-3".to_string() == String::from("-3"));

                
        // self.should_shoot = true;
    }

    // fn stop_shooting(&mut self) {
    //     self.should_shoot = false;
    // }

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
        
        // if self.should_shoot {
        //     self.should_shoot = false;
        // }
    }
}
