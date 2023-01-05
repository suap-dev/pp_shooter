use console_engine::ConsoleEngine;
use console_engine::KeyCode;

const LEFT: usize = 0;
const RIGHT: usize = 1;

const PP: [&str; 2] = ["c==3", "Ɛ==ↄ"];

fn main() {
    let mut engine = ConsoleEngine::init_fill(15).unwrap();
   
    // pp coords
    let mut pp_x = 0;
    let mut pp_y = 0;

    // pp facing
    let mut facing = LEFT;

    loop {   
        // this clears everything
        engine.clear_screen();

        // we are going to create a picture
        engine.print(
            pp_x,
            pp_y,
            PP[facing]
        );

        // here we are drawing created picture
        engine.draw();

        // pausing program till the next frame, so we can...
        engine.wait_frame();

        // ...handle the logic, keys, maths, etc.
        if engine.is_key_held(KeyCode::Right) {
            facing = RIGHT;
            pp_x += 1;
        }
        if engine.is_key_held(KeyCode::Left) {
            facing = LEFT;
            pp_x = pp_x - 1;
        }
        if engine.is_key_held(KeyCode::Up) {
            pp_y = pp_y - 1;
        }
        if engine.is_key_held(KeyCode::Down) {
            pp_y = pp_y + 1;
        }
        if engine.is_key_pressed(KeyCode::Esc) {
            break;
        }
    }
}
