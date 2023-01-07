use console_engine::{ConsoleEngine, KeyCode};

// TODO: Make the shooting mechanism work correctly... Make the projectile move.

fn main() {
    let mut screen = ConsoleEngine::init_fill(30).unwrap();
    let mut screen = ConsoleEngine::init_fill(15).unwrap();

    let mut bullet: Projectile = Projectile {
        position: Coords { x: 1, y: 1},
        model: String::from("-")    // length of this string should be taken into consideration
    };

    let mut p1 = Player {
        model: [String::from("c==3"), String::from("Ɛ==ↄ")],
        facing: Facing::Left,
        position: Coords { x: 8, y: 4 },
        should_shoot: false,
    };

    

    loop {
        // this clears everything
        screen.clear_screen();

        // we are going to create a picture
        p1.add_to_frame(&mut screen);
        bullet.add_to_frame(&mut screen);

        // here we are drawing created picture
        screen.draw();

        // pausing program till the next frame, so we can...
        screen.wait_frame();

        // ...handle the logic, keys, maths, etc.
        if screen.is_key_held(KeyCode::Right) {
            p1.go_right();
        }
        if screen.is_key_held(KeyCode::Left) {
            p1.go_left();
        }
        if screen.is_key_held(KeyCode::Up) {
            p1.go_up();
        }
        if screen.is_key_held(KeyCode::Down) {
            p1.go_down();
        }
        if screen.is_key_released(KeyCode::Enter) {
            p1.shoot(&mut bullet);
        }
        if screen.is_key_pressed(KeyCode::Esc) {
            break;
        }
    }
}

struct Coords {
    x: i32,
    y: i32,
}

#[derive(Clone, Copy)]
enum Facing {
    Left,
    Right,
}

struct Projectile {
    position: Coords,
    model: String,
}

impl Projectile {
    fn add_to_frame(&mut self, screen: &mut ConsoleEngine) {
        screen.print(
            self.position.x,
            self.position.y,
            self.model.as_str(),
        );
    }
}

struct Player {
    model: [String; 2],
    facing: Facing,
    position: Coords,
    should_shoot: bool,
}
impl Player {
    fn go_right(&mut self) {
        self.position.x += 1;
        self.facing = Facing::Right;
    }

    fn go_left(&mut self) {
        self.position.x -= 1;
        self.facing = Facing::Left;
    }

    fn go_up(&mut self) {
        self.position.y -= 1;
    }

    fn go_down(&mut self) {
        self.position.y += 1;
    }

    fn shoot(&mut self, projectile: &mut Projectile) {
        projectile.position = self.get_barrel_exit_coords();
        self.should_shoot = true;
    }

    // fn stop_shooting(&mut self) {
    //     self.should_shoot = false;
    // }

    fn get_barrel_exit_coords(&self) -> Coords {
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

    fn add_to_frame(&mut self, screen: &mut ConsoleEngine) {
        screen.print(
            self.position.x,
            self.position.y,
            self.model[self.facing as usize].as_str(),
        );

        // TODO: why this is not important?
        // if self.should_shoot {
        //     self.should_shoot = false;
        // }
    }
}