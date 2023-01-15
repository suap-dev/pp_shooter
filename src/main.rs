pub mod engine;
pub mod entities;

use console_engine::{Color, KeyCode};

use entities::{actor::*, model::*, *};
use engine::{pixel::Pixel, vector2f::Vec2f};

fn main() {
    let mut screen = engine::MyEngine::init(80, 30, 15);

    let center = Vec2f::new(
        screen.get_width() as f32 / 2.0,
        screen.get_height() as f32 / 2.0,
    );

    let body_color = Color::DarkGrey;
    let eyes_color = Color::White;

    let mut a: Actor = Actor::from_model(Model {
        pixels: vec![
            Pixel::new(body_color, Vec2f::inew(0, 0)),
            Pixel::new(body_color, Vec2f::inew(-1, 0)),
            Pixel::new(body_color, Vec2f::inew(1, 0)),
            Pixel::new(body_color, Vec2f::inew(0, -1)),
            Pixel::new(body_color, Vec2f::inew(0, -2)),
            Pixel::new(body_color, Vec2f::inew(-1, -2)),
            Pixel::new(body_color, Vec2f::inew(1, -2)),
            Pixel::new(body_color, Vec2f::inew(0, -3)),
            Pixel::new(eyes_color, Vec2f::inew(-1, -3)),
            Pixel::new(eyes_color, Vec2f::inew(1, -3)),
            Pixel::new(body_color, Vec2f::inew(0, -4)),
            Pixel::new(body_color, Vec2f::inew(-1, -4)),
            Pixel::new(body_color, Vec2f::inew(1, -4)),
            Pixel::new(body_color, Vec2f::inew(0, 1)),
            Pixel::new(body_color, Vec2f::inew(-1, 2)),
            Pixel::new(body_color, Vec2f::inew(1, 2)),
        ],
    });

    a.set_position(center);

    loop {
        screen.clear();

        screen.add_to_frame(&a);

        screen.update();

        screen.wait_frame();

        if screen.key(KeyCode::Right) {
            a.translate(Vec2f::inew(1, 0));
        }
        if screen.key(KeyCode::Left) {
            a.translate(Vec2f::inew(-1, 0));
        }
        if screen.key(KeyCode::Up) {
            a.translate(Vec2f::inew(0, -1));
        }
        if screen.key(KeyCode::Down) {
            a.translate(Vec2f::inew(0, 1));
        }

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
