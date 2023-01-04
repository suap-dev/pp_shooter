use console_engine::ConsoleEngine;
use console_engine::KeyCode;

fn main() {
    let mut engine = ConsoleEngine::init_fill(15).unwrap();
    
    // playable persona
    let pp = "c==3";

    // pp coords
    let mut pp_x = 0;
    let mut pp_y = 0;

    loop {   
        // this clears everything
        engine.clear_screen();

        // we are going to create a picture
        engine.print(
            pp_x,
            pp_y,
            pp
        );

        // here we are drawing created picture
        engine.draw();

        // pausing program till the next frame, so we can...
        engine.wait_frame();
        // ...handle the logic, keys, maths, etc.
        if engine.is_key_pressed(KeyCode::Right) {
            pp_x = pp_x + 1;
        }
        if engine.is_key_pressed(KeyCode::Left) {
            pp_x = pp_x - 1;
        }
        if engine.is_key_pressed(KeyCode::Up) {
            pp_y = pp_y - 1;
        }
        if engine.is_key_pressed(KeyCode::Down) {
            pp_y = pp_y + 1;
        }
        if engine.is_key_pressed(KeyCode::Esc) {
            break;
        }
    }
}
