use console_engine::{ ConsoleEngine, KeyCode };

// TODO: Make the shooting mechanism work correctly... Now it just spawns a projectile attached to the tip.

fn main() {
    let mut screen = ConsoleEngine::init_fill(30).unwrap();

    let mut p1 = Player{
        model: [String::from("c==3"), String::from("Ɛ==ↄ")],
        facing: Facing::Left,
        position: Coords{x: 1, y: 2},
        projectile: String::from("-"),  // length of this string should be taken into consideration
                                        // when shooting
        is_shooting: false,
    };

    loop {   
        // this clears everything
        screen.clear_screen();

        // we are going to create a picture
        p1.add_to_picture(&mut screen);
       
        // here we are drawing created picture
        screen.draw();

        // pausing program till the next frame, so we can...
        screen.wait_frame();

        // ...handle the logic, keys, maths, etc.
        if screen.is_key_held(KeyCode::Right) { p1.go_right(); }
        if screen.is_key_held(KeyCode::Left) { p1.go_left(); }
        if screen.is_key_held(KeyCode::Up) { p1.go_up(); }
        if screen.is_key_held(KeyCode::Down) { p1.go_down(); }
        if screen.is_key_held(KeyCode::Enter) { p1.start_shooting(); }
        if screen.is_key_pressed(KeyCode::Esc) { break; }
    }
}

struct Player {
    model: [String; 2],
    projectile: String,
    facing: Facing,
    position: Coords,
    is_shooting: bool,
}
impl Player {
    fn go_right(&mut self){
        self.position.x += 1;
        self.facing = Facing::Right;
    }

    fn go_left(&mut self){
        self.position.x -= 1;
        self.facing = Facing::Left;
    }

    fn go_up(&mut self){
        self.position.y -= 1;
    }

    fn go_down(&mut self){
        self.position.y += 1;
    }
    
    fn start_shooting(&mut self){
        self.is_shooting = true;
    }

    fn stop_shooting(&mut self){
        self.is_shooting = false;
    }
    
    fn get_barrel_exit_coords(&self) -> Coords{
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

        Coords{x, y}
    }

    fn add_to_picture(&self, screen: &mut ConsoleEngine){
        screen.print(
            self.position.x,
            self.position.y,
            self.model[self.facing as usize].as_str(),
        );

        let barrel_exit = self.get_barrel_exit_coords();
        screen.print(
            barrel_exit.x,
            barrel_exit.y,
            self.projectile.as_str(),
        );
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
