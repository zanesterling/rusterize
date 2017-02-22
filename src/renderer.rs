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

    offset_x: Coord,
    offset_y: Coord,
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

            offset_x: 0,
            offset_y: 0,
        }
    }

    pub fn draw_line(
        &mut self,
        x1: Coord,
        y1: Coord,
        x2: Coord,
        y2: Coord
    ) {
        let x1 = x1 + self.offset_x;
        let x2 = x2 + self.offset_x;
        let y1 = y1 + self.offset_y;
        let y2 = y2 + self.offset_y;

        let dx = x2 as i64 - x1 as i64;
        let dy = y2 as i64 - y1 as i64;
        let adx = if dx >= 0 { dx } else { -dx };
        let ady = if dy >= 0 { dy } else { -dy };

        let x_step = if x2 > x1 { 1 } else { -1 };
        let y_step = if y2 > y1 { 1 } else { -1 };
        let mut x = x1;
        let mut y = y1;
        let mut error: i64 = 0;
        loop {
            if adx >= ady {
                if 2 * error > adx {
                    y += y_step;
                    error -= adx;
                }
                error += ady;
            } else {
                if 2 * error > ady {
                    x += x_step;
                    error -= ady;
                }
                error += adx;
            }

            self.texture.set_pixel(x, y, pixel::WHITE);

            if adx >= ady { x += x_step }
            else          { y += y_step }

            if x == x2 { break }
        }
    }

    pub fn translate(&mut self, dx: Coord, dy: Coord) {
        self.offset_x += dx;
        self.offset_y += dy;
    }

    pub fn clear(&mut self) {
        self.texture.set_all_pixels(pixel::BLACK);
        self.offset_x = 0;
        self.offset_y = 0;
    }

    pub fn display(&mut self) -> Result<(), Box<error::Error>> {
        self.screen.display_texture(&self.texture)
    }
}
