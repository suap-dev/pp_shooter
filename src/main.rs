mod data_structs;
mod entities;

use data_structs::{Direction, Facing, Vec2};
use entities::{Player, Projectile, ProjectileHandler};

use console_engine::{ConsoleEngine, KeyCode};

fn main() {
    let mut screen = ConsoleEngine::init_fill(60).unwrap();

    let mut p1 = Player {
        model: [String::from("c==3"), String::from("Ɛ==ↄ")],
        facing: Facing::Right,
        position: Vec2::inew(8, 4),
        velocity: Vec2::zero(),
        base_speed: 0.2,
        projectile_type: Projectile::new([String::from("=~-"), String::from("-~=")], 2.0, 120.0),
    };

    let mut projectile_handler = ProjectileHandler::new();

    loop {
        // this clears everything
        screen.clear_screen();

        // we are going to create a picture
        projectile_handler.update_frame(&mut screen);
        p1.update_frame(&mut screen);

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
        if screen.is_key_released(KeyCode::Enter) {
            projectile_handler.handle(p1.shoot());
        }
        if screen.is_key_pressed(KeyCode::Esc) {
            break;
        }
    }
}
