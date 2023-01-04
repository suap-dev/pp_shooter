use console_engine::ConsoleEngine;
use console_engine::KeyCode;

fn main() {
    let mut engine = ConsoleEngine::init_fill(2).unwrap();

    loop {
        // ConsoleEngine::wait_frame(&mut engine);
        engine.wait_frame();
        // if ConsoleEngine::is_key_pressed(&engine, KeyCode::Esc){
        if engine.is_key_pressed(KeyCode::Esc) {
            break;
        }
    }

        
    println!("Hello, world!");


}
