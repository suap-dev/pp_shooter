pub mod pixel;
pub mod vector2f;

use console_engine::{ConsoleEngine, KeyCode};
use pixel::Pixel;
use vector2f::Vec2f;

pub struct MyEngine {
    my_console: ConsoleEngine,
}

impl MyEngine {
    pub fn init(width: u32, height: u32, target_fps: u32) -> Self {
        Self {
            my_console: ConsoleEngine::init(width, height, target_fps).unwrap(),
        }
    }

    // FIXME: a problem will occur here... :]
    pub fn set_pixel(&mut self, position: Vec2f, pixel: Pixel) {
        let px = console_engine::pixel::pxl_fbg(' ', pixel.color, pixel.color);
        let x = position.x() * 2.0;
        self.my_console.set_pxl(x as i32, position.y() as i32, px);
        self.my_console
            .set_pxl((x + 1.0) as i32, position.y() as i32, px);
    }

    // pub fn set_pixel_2(&mut self, position: Vec2f, pixel: &Pixel) {
    //     let px = console_engine::pixel::pxl_fbg(' ', pixel.color, pixel.color);

    //     let position = position + pixel.offset;
    //     let x = position.x() * 2.0;
    //     self.my_console.set_pxl(x as i32, position.y() as i32, px);
    //     self.my_console
    //         .set_pxl((x + 1.0) as i32, position.y() as i32, px);
    // }

    pub fn key(&self, key: KeyCode) -> bool {
        self.my_console.is_key_pressed(key)
    }

    pub fn wait_frame(&mut self) {
        self.my_console.wait_frame();
    }

    pub fn clear(&mut self) {
        self.my_console.clear_screen();
    }

    pub fn update(&mut self) {
        self.my_console.draw();
    }

    pub fn get_height(&self) -> u32 {
        self.my_console.get_height()
    }

    // FIXME: there is something that is going to fuck up here. :]
    pub fn get_width(&self) -> u32 {
        self.my_console.get_width()
    }
}
