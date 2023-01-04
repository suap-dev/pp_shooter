use console_engine::ConsoleEngine;
use console_engine::KeyCode;

fn main() {
    let mut engine = ConsoleEngine::init_fill(15).unwrap();
    engine.print(0,0, "Hello, world!");
    engine.draw();

    loop {   
        // to czyści ekran ze wszystkiego
        engine.clear_screen();

        // tutaj będziemy tworzyć rysunek
        engine.print(0,0, "Siusiak");

        // tutaj rysujemy stworzony rysunek
        engine.draw();

        // wstrzymujemy bieg programu do następnej klatki, by...
        engine.wait_frame();
        // ...obsłużyć logikę, klawisze, matematykę, itp:
        if engine.is_key_pressed(KeyCode::Esc) {
            break;
        }
    }
}
