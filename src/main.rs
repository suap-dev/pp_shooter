pub mod engine;
pub mod entities;

use console_engine::{Color, KeyCode};
use engine::{pixel::Pixel, vector2f::Vec2f};

fn main() {
    let mut screen = engine::MyEngine::init(30, 15, 15);

    // let px_list = vec![
    //     Pixel::new(Color::Magenta, Vec2f::inew(0, 0)),
    //     Pixel::new(Color::Magenta, Vec2f::inew(1, 0)),
    //     Pixel::new(Color::Magenta, Vec2f::inew(1, 1)),
    //     Pixel::new(Color::Magenta, Vec2f::inew(1, 2)),
    // ];
    
    // let center = Vec2f::new(
    //     screen.get_width() as f32 / 4.0,
    //     screen.get_height() as f32 / 2.0,
    // );


    loop {
        screen.clear();

        screen.set_pixel(Vec2f::inew(1, 1), Pixel::from_color(Color::Magenta));

        // for px in &px_list {
        //     screen.set_pixel_2(center, px);
        // }

        screen.update();

        screen.wait_frame();

        if screen.key(KeyCode::Esc) {
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
