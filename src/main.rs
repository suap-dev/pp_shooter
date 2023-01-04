use console_engine::ConsoleEngine;
use console_engine::KeyCode;

fn main() {
    let mut engine = ConsoleEngine::init_fill(2).unwrap();

    loop {
        engine.wait_frame();
        if engine.is_key_pressed(KeyCode::Esc) {
            break;
        }
    }

        
    println!("Hello, world!");


}
