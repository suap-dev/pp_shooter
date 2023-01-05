use console_engine::{ ConsoleEngine, KeyCode };


fn main() {
    let mut engine = ConsoleEngine::init_fill(30).unwrap();

    let mut pp = Player{
        model: [String::from("c==3"), String::from("Ɛ==ↄ")],
        facing: Facing::Left,
        position: Coords{x: 1, y: 2}
    };       

    loop {   
        // this clears everything
        engine.clear_screen();

        // we are going to create a picture
        pp.draw(&mut engine);
       
        // here we are drawing created picture
        engine.draw();

        // pausing program till the next frame, so we can...
        engine.wait_frame();

        // ...handle the logic, keys, maths, etc.
        if engine.is_key_held(KeyCode::Right) { pp.go_right(); }
        if engine.is_key_held(KeyCode::Left) { pp.go_left(); }
        if engine.is_key_held(KeyCode::Up) { pp.go_up(); }
        if engine.is_key_held(KeyCode::Down) { pp.go_down(); }
        if engine.is_key_pressed(KeyCode::Esc) { break; }
    }
}

struct Player {
    model: [String; 2],
    facing: Facing,
    position: Coords,
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

    fn draw(&self, engine: &mut ConsoleEngine){
        engine.print(
            self.position.x,
            self.position.y,
            self.model[self.facing as usize].as_str() ,
        );
    }
}

struct Coords {
    x: i32,
    y: i32,
}

#[derive(Clone, Copy)]
enum Facing {
    Left = 0,
    Right = 1,
} 