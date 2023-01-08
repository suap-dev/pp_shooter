mod data_structs;
mod entities;

use data_structs::{Direction, Facing, Vec2};
use entities::{Player, Projectile};

use console_engine::{ConsoleEngine, KeyCode};

// TODO: Make the shooting mechanism work correctly... Make the projectile move.

fn main() {
    let mut screen = ConsoleEngine::init_fill(60).unwrap();

    let mut bullet: Projectile = Projectile {
        position: Vec2 { x: -1., y: -1. },
        model: String::from("--"), // length of this string should be taken into consideration
        velocity: Vec2 { x: 0., y: 0. },
        speed: 1.0,
    };

    let mut p1 = Player {
        model: [String::from("c==3"), String::from("Ɛ==ↄ")],
        facing: Facing::Right,
        direction: Direction::Right,
        position: Vec2 { x: 8., y: 4. },
        velocity: Vec2 { x: 0., y: 0. },
        should_shoot: false,
    };

    loop {
        // this clears everything
        screen.clear_screen();

        // we are going to create a picture
        bullet.add_to_frame(&mut screen);
        p1.add_to_frame(&mut screen);

        // here we are drawing created picture
        screen.draw();

        // pausing program till the next frame, so we can...
        screen.wait_frame();

        if screen.is_key_pressed(KeyCode::Up) {
            p1.go(Direction::Up);
        }
        if screen.is_key_pressed(KeyCode::Down) {
            p1.go(Direction::Down);
        }
        if screen.is_key_pressed(KeyCode::Right) {
            p1.go(Direction::Right);
        }
        if screen.is_key_pressed(KeyCode::Left) {
            p1.go(Direction::Left);
        }
        if screen.is_key_pressed(KeyCode::Char(' ')) {
            p1.stop();
        }

        // ...handle the logic, keys, maths, etc.
        // if screen.is_key_held(KeyCode::Right) {
        //     p1.go_right();
        // }
        // if screen.is_key_held(KeyCode::Left) {
        //     p1.go_left();
        // }
        // if screen.is_key_held(KeyCode::Up) {
        //     p1.go_up();
        // }
        // if screen.is_key_held(KeyCode::Down) {
        //     p1.go_down();
        // }

        if screen.is_key_released(KeyCode::Enter) {
            p1.shoot(&mut bullet);
        }
        if screen.is_key_pressed(KeyCode::Esc) {
            break;
        }

        // game logic
        p1.proceed_in_time();
        bullet.proceed_in_time();
    }
}
