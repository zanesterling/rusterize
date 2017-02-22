use std::error;

use pixel;
use screen::Screen;
use texture::Texture;
use types::*;


pub struct Renderer<S>
    where S: Screen
{
    screen: S,
    texture: Texture,
}

impl<S> Renderer<S>
    where S: Screen
{
    pub fn new(screen: S) -> Renderer<S> {
        let w = screen.width();
        let h = screen.height();

        Renderer {
            screen: screen,
            texture: Texture::new(w, h),
        }
    }

    pub fn draw_line(
        &mut self,
        x1: Coord,
        y1: Coord,
        x2: Coord,
        y2: Coord
    ) {
        let dx = x2 as i64 - x1 as i64;
        let dy = y2 as i64 - y1 as i64;
        let adx = if dx >= 0 { dx } else { -dx };
        let ady = if dy >= 0 { dy } else { -dy };

        if adx < ady { panic!("tall lines not yet handled"); }

        let x_step = if x2 > x1 { 1 } else { -1 };
        let y_step = if y2 > y1 { 1 } else { -1 };
        let mut x = x1;
        let mut y = y1;
        let mut error: i64 = 0;
        loop {
            if 2 * error > adx {
                y += y_step;
                error -= adx;
            }
            error += ady;

            self.texture.set_pixel(x, y, pixel::WHITE);

            x += x_step;
            if x == x2 { break }
        }
    }

    pub fn clear(&mut self) {
        self.texture.set_all_pixels(pixel::BLACK);
    }

    pub fn display(&mut self) -> Result<(), Box<error::Error>> {
        self.screen.display_texture(&self.texture)
    }
}
