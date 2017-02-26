use std::cmp::Ordering::Equal;
use std::error;
use std::mem;

use pixel;
use pixel::Pixel;
use screen::Screen;
use texture::Texture;
use types::*;


pub struct Renderer<S>
    where S: Screen
{
    screen: S,
    texture: Texture,

    transform: Transform,
    color: Pixel,
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

            transform: Transform::identity(),
            color: pixel::WHITE,
        }
    }

    pub fn draw_point(&mut self, p: Point) {
        let p = p * self.transform;
        let d = 7;
        for row in 0 .. d {
            self.texture.set_row(
                p.x as PixCoord - d / 2,
                p.x as PixCoord + d / 2,
                p.y as PixCoord + row - d / 2,
                self.color
            );
        }
    }

    fn draw_point_with_transform(&mut self, p: Point, transform: Transform) {
        let old_transform = self.transform;
        self.transform = transform;
        self.draw_point(p);
        self.transform = old_transform;
    }

    pub fn draw_line(&mut self, p1: Point, p2: Point) {
        let p1 = p1 * self.transform;
        let p2 = p2 * self.transform;
        let p1x = p1.x as PixCoord;
        let p1y = p1.y as PixCoord;
        let p2x = p2.x as PixCoord;
        let p2y = p2.y as PixCoord;

        let dx = p2.x as i64 - p1.x as i64;
        let dy = p2.y as i64 - p1.y as i64;
        let adx = if dx >= 0 { dx } else { -dx };
        let ady = if dy >= 0 { dy } else { -dy };

        let x_step = if p2x > p1x { 1 } else { -1 };
        let y_step = if p2y > p1y { 1 } else { -1 };
        let mut x = p1x;
        let mut y = p1y;
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

            self.texture.set_pixel(x, y, self.color);

            if adx >= ady {
                if x == p2x { break }
                else { x += x_step }
            } else {
                if y == p2y { break }
                else { y += y_step }
            }
        }
    }

    fn draw_line_with_transform(
        &mut self,
        p1: Point,
        p2: Point,
        transform: Transform
    ) {
        let old_transform = self.transform;
        self.transform = transform;
        self.draw_line(p1, p2);
        self.transform = old_transform;
    }

    pub fn fill_triangle(&mut self, t: Triangle) {
        let t = t * self.transform;
        let mut pts = t.to_arr();
        pts.sort_by(
            |p1, p2|
            p1.y.partial_cmp(&p2.y)
                .unwrap_or(Equal)
        );
        let (top, middle, bot) = (pts[0], pts[1], pts[2]);

        if      top.y == middle.y { self.fill_top_flat_triangle(t); }
        else if middle.y == bot.y { self.fill_bottom_flat_triangle(t); }
        else {
            let dy_middle = (middle.y - top.y) as f64;
            let dy_bot = (bot.y - top.y) as f64;
            let dx_bot = (bot.x - top.x) as f64;
            let dz_bot = (bot.z - top.z) as f64;

            let v4 = pt![
                top.x + ((dy_middle / dy_bot) * dx_bot) as Coord,
                middle.y,
                top.z + ((dy_middle / dy_bot) * dz_bot) as Coord
            ];
            self.fill_bottom_flat_triangle(trigon![top, middle, v4]);
            self.fill_top_flat_triangle(trigon![middle, v4, bot]);
        }
    }

    fn fill_bottom_flat_triangle(&mut self, t: Triangle) {
        let (top, mut left, mut right) = t.to_tuple();
        if left.x > right.x { mem::swap(&mut left, &mut right) }
        let invslope1 = (left.x - top.x)  / (left.y - top.y);
        let invslope2 = (right.x - top.x) / (right.y - top.y);
        let mut curx1 = top.x;
        let mut curx2 = top.x;

        for y in top.y as PixCoord .. left.y as PixCoord {
            self.texture.set_row(
                curx1 as PixCoord,
                curx2 as PixCoord,
                y,
                self.color
            );
            curx1 += invslope1;
            curx2 += invslope2;
        }

        self.texture.set_row(
            left.x  as PixCoord,
            right.x as PixCoord,
            left.y  as PixCoord,
            self.color
        );
    }

    fn fill_top_flat_triangle(&mut self, t: Triangle) {
        let (mut left, mut right, bot) = t.to_tuple();
        if left.x > right.x { mem::swap(&mut left, &mut right) }
        let invslope1 = (bot.x - left.x)  / (bot.y - left.y);
        let invslope2 = (bot.x - right.x) / (bot.y - right.y);
        let mut curx1 = left.x;
        let mut curx2 = right.x;

        for y in left.y as PixCoord .. bot.y as PixCoord + 1 {
            self.texture.set_row(
                curx1 as PixCoord,
                curx2 as PixCoord,
                y,
                self.color
            );
            curx1 += invslope1;
            curx2 += invslope2;
        }
    }

    pub fn clear(&mut self) {
        self.texture.set_all_pixels(pixel::BLACK);
    }

    pub fn display(&mut self) -> Result<(), Box<error::Error>> {
        self.screen.display_texture(&self.texture)
    }


    pub fn set_transform(&mut self, t: Transform) {
        self.transform = t;
    }

    pub fn clear_transform(&mut self) {
        self.transform = Transform::identity();
    }

    pub fn translate(&mut self, p: Point) {
        self.transform = Transform::translate(p) * self.transform;
    }

    pub fn rotate_x(&mut self, theta: f64) {
        self.transform = Transform::rotate_x(theta) * self.transform;
    }

    pub fn rotate_y(&mut self, theta: f64) {
        self.transform = Transform::rotate_y(theta) * self.transform;
    }

    pub fn rotate_z(&mut self, theta: f64) {
        self.transform = Transform::rotate_z(theta) * self.transform;
    }

    pub fn scale(&mut self, x: f64, y: f64, z: f64) {
        self.transform = Transform::scale(x, y, z) * self.transform;
    }

    pub fn perspective(&mut self) {
        self.transform = Transform::perspective() * self.transform;
    }


    pub fn set_color(&mut self, color: Pixel) { self.color = color; }
}
