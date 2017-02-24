use std::error;
use std::mem;

use pixel;
use screen::Screen;
use texture::Texture;
use types::*;


pub struct Renderer<S>
    where S: Screen
{
    screen: S,
    texture: Texture,

    offset: Point,
}

#[allow(dead_code)]
impl<S> Renderer<S>
    where S: Screen
{
    pub fn new(screen: S) -> Renderer<S> {
        let w = screen.width();
        let h = screen.height();

        Renderer {
            screen: screen,
            texture: Texture::new(w, h),

            offset: pt![0, 0],
        }
    }

    pub fn draw_point(&mut self, p: Point) {
        println!("{:?}", p);
        let p = p + self.offset;
        let d = 7;
        for row in 0 .. d {
            self.texture.set_row(
                p.x - d / 2,
                p.x + d / 2,
                p.y + row - d / 2,
                pixel::WHITE,
            );
        }
    }

    fn draw_point_with_offset(&mut self, p: Point, offset: Point) {
        let old_offset = self.offset;
        self.offset = offset;
        self.draw_point(p);
        self.offset = old_offset;
    }

    pub fn draw_line(&mut self, p1: Point, p2: Point) {
        let p1 = p1 + self.offset;
        let p2 = p2 + self.offset;

        let dx = p2.x as i64 - p1.x as i64;
        let dy = p2.y as i64 - p1.y as i64;
        let adx = if dx >= 0 { dx } else { -dx };
        let ady = if dy >= 0 { dy } else { -dy };

        let x_step = if p2.x > p1.x { 1 } else { -1 };
        let y_step = if p2.y > p1.y { 1 } else { -1 };
        let mut x = p1.x;
        let mut y = p1.y;
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

            if adx >= ady {
                if x == p2.x { break }
                else { x += x_step }
            } else {
                if y == p2.y { break }
                else { y += y_step }
            }
        }
    }

    fn draw_line_with_offset(
        &mut self,
        p1: Point,
        p2: Point,
        offset: Point
    ) {
        let old_offset = self.offset;
        self.offset = offset;
        self.draw_line(p1, p2);
        self.offset = old_offset;
    }

    pub fn fill_triangle(&mut self, mut t: Triangle) {
        t.sort_by_key(|p| p.y);
        let off = self.offset;
        let (top, middle, bot) = (t[0] + off, t[1] + off, t[2] + off);

        if top.y == middle.y      { self.fill_top_flat_triangle(t); }
        else if middle.y == bot.y { self.fill_bottom_flat_triangle(t); }
        else {
            let dy_middle = (middle.y - top.y) as f64;
            let dy_bot = (bot.y - top.y) as f64;
            let dx_bot = (bot.x - top.x) as f64;

            let v4 = Point {
                x: top.x + ((dy_middle / dy_bot) * dx_bot) as Coord,
                y: middle.y,
            };
            self.fill_top_flat_triangle(trigon![top, middle, v4]);
            self.fill_bottom_flat_triangle(trigon![middle, v4, bot]);
        }
    }

    fn fill_top_flat_triangle(&mut self, t: Triangle) {
        let (top, mut left, mut right) = (t[0], t[1], t[2]);
        if left.x > right.x { mem::swap(&mut left, &mut right) }
        let invslope1 = (left.x - top.x) as f64 / (left.y - top.y) as f64;
        let invslope2 = (right.x - top.x) as f64 / (right.y - top.y) as f64;
        let mut curx1 = top.x as f64;
        let mut curx2 = top.x as f64;

        for y in top.y .. left.y + 1 {
            self.texture.set_row(
                curx1 as Coord,
                curx2 as Coord,
                y,
                pixel::WHITE
            );
            curx1 += invslope1;
            curx2 += invslope2;
        }
    }

    fn fill_bottom_flat_triangle(&mut self, t: Triangle) {
        let (mut left, mut right, bot) = (t[0], t[1], t[2]);
        if left.x > right.x { mem::swap(&mut left, &mut right) }
        let invslope1 = (bot.x - left.x)  as f64 / (bot.y - left.y)  as f64;
        let invslope2 = (bot.x - right.x) as f64 / (bot.y - right.y) as f64;
        let mut curx1 = left.x  as f64;
        let mut curx2 = right.x as f64;

        for y in left.y .. bot.y + 1 {
            self.texture.set_row(
                curx1 as Coord,
                curx2 as Coord,
                y,
                pixel::WHITE
            );
            curx1 += invslope1;
            curx2 += invslope2;
        }
    }

    pub fn translate(&mut self, dx: Coord, dy: Coord) {
        self.offset.x += dx;
        self.offset.y += dy;
    }

    pub fn clear(&mut self) {
        self.texture.set_all_pixels(pixel::BLACK);
        self.offset.x = 0;
        self.offset.y = 0;
    }

    pub fn display(&mut self) -> Result<(), Box<error::Error>> {
        self.screen.display_texture(&self.texture)
    }
}
