pub mod entities;
pub mod engine;

use engine::{vector2f::Vec2f, pixel::Pixel};
use console_engine::{Color, KeyCode};


fn main() {
    let mut screen = engine::MyEngine::init(30, 15, 15);
    
    loop {
        screen.clear();

        screen.set_pixel(
            Vec2f::inew(1, 1),            
            Pixel::from_color(Color::Magenta),
        );

        screen.update();

        screen.wait_frame();

        if screen.key(KeyCode::Esc){
            break;
        }
    }
}


// let mut screen;
// screen = ConsoleEngine::init(30, 15, 15).unwrap();

// // let a1 = Actor::from_model(Model::from_string("<3".to_string()));
// let mut p1 = Pawn::new(&screen, Model::new_pixel(Color::Magenta), Vec2f::new(0.5, 0.8));
// p1.set_position(Vec2f::inew(5, 9));



// loop {        
//     screen.clear_screen();

//     // a1.add_to_frame(&mut screen);
//     p1.add_to_frame(&mut screen);

//     screen.draw();        

//     screen.wait_frame();
//     if screen.is_key_pressed(KeyCode::Esc) {
//         break;
//     }
    
//     // p1.update();
