mod data_structs;
mod entities;

use data_structs::{Coords, Facing};
use entities::{Player, Projectile};

use console_engine::{ConsoleEngine, KeyCode};

fn main() {
    let mut screen = ConsoleEngine::init_fill(30).unwrap();
    // Vec needs to know what type of data it stores

    let mut projectiles: Vec<Projectile> = Vec::new();

    let bullet: Projectile = Projectile {
        position: Coords { x: -1, y: -1 },
        model: String::from("-"), // length of this string should be taken into consideration
        velocity: 1,
        valid: true,
    };

    let mut p1 = Player {
        model: [String::from("c==3"), String::from("Ɛ==ↄ")],
        facing: Facing::Left,
        position: Coords { x: 8, y: 4 },
        should_shoot: false,
    };

    loop {
        // if projectile is valid -> retain it
        // let is_valid = |x: &Projectile| x.valid;
        // projectiles.retain(is_valid);

        projectiles.retain(|x| x.valid);

        // TODO: when I grow up I will do it through retain_mut

        // this clears everything
        screen.clear_screen();

        // we are going to create a picture
        p1.update_frame(&mut screen);
        for projectile in &mut projectiles {
            projectile.update_frame(&mut screen);
        }
        screen.print(1, 1, projectiles.len().to_string().as_str());

        // here we are drawing created picture
        screen.draw();

        // pausing program till the next frame, so we can...
        screen.wait_frame();

        // ...handle the logic, keys, maths, etc.
        if screen.is_key_held(KeyCode::Right) {
            p1.go_right();
        }
        if screen.is_key_held(KeyCode::Left) {
            p1.go_left();
        }
        if screen.is_key_held(KeyCode::Up) {
            p1.go_up();
        }
        if screen.is_key_held(KeyCode::Down) {
            p1.go_down();
        }
        if screen.is_key_released(KeyCode::Enter) {
            let bullet1 = p1.shoot(&bullet);
            projectiles.push(bullet1);
        }
        if screen.is_key_pressed(KeyCode::Esc) {
            break;
        }
    }
}
