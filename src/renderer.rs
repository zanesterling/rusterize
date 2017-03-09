use std::cmp::Ordering::Equal;
use std::error;
use std::f64;
use std::mem;

use pixel;
use pixel::Pixel;
use screen::Screen;
use texture::Texture;
use types::*;


macro_rules! do_with_color {
    ( $renderer:expr, $color:expr, $block:block ) => {
        let old_color = $renderer.color;
        $renderer.color = $color;
        $block;
        $renderer.color = old_color;
    }
}


pub enum LightingMode {
    NoShading,
    FlatShading,
}

pub struct Renderer<S>
    where S: Screen
{
    screen: S,
    texture: Texture,

    transform: Transform,
    color: Pixel,

    light: Point,
    lighting_mode: LightingMode,
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

            light: pt![0., 0., 0.],
            lighting_mode: LightingMode::NoShading,
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
                f64::NEG_INFINITY,
                f64::NEG_INFINITY,
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

            // FIXME: Do depth lerping.
            self.texture.set_pixel(
                x,
                y,
                f64::INFINITY,
                self.color
            );

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

    pub fn draw_triangle(&mut self, t: Triangle) {
        self.draw_line(t.p1, t.p2);
        self.draw_line(t.p2, t.p3);
        self.draw_line(t.p3, t.p1);
    }

    pub fn fill_triangle(&mut self, t: Triangle) {
        // Backface culling.
        let centroid = (t.p1 + t.p2 + t.p3) * (1. / 3.);
        let ct = t * self.transform;
        if ct.normal().dot(centroid) >= 0. { return }

        // Sort points by y coord.
        let mut pts = ct.to_arr();
        pts.sort_by(
            |p1, p2|
            p1.y.partial_cmp(&p2.y)
                .unwrap_or(Equal)
        );
        let (top, middle, bot) = (pts[0], pts[1], pts[2]);

        // Compute color of triangle based on light.
        let old_color = self.color;
        self.color = self.light_triangle(t, old_color);

        const EPSILON: f64 = 1.;
        if middle.y - top.y < EPSILON {
            self.fill_top_flat_triangle(Triangle::from_arr(pts));
        } else if bot.y - middle.y < EPSILON {
            self.fill_bottom_flat_triangle(Triangle::from_arr(pts));
        } else {
            let dy_mid: Coord = middle.y - top.y;
            let dy_bot: Coord = bot.y - top.y;
            let dx_bot: Coord = bot.x - top.x;
            let dz_bot: Coord = bot.z - top.z;

            let v4 = pt![
                top.x + dx_bot * dy_mid / dy_bot,
                middle.y,
                top.z + dz_bot * dy_mid / dy_bot
            ];
            self.fill_bottom_flat_triangle(trigon![top, middle, v4]);
            self.fill_top_flat_triangle(trigon![middle, v4, bot]);
        }

        self.color = old_color;
    }

    fn light_triangle(&self, t: Triangle, color: Pixel) -> Pixel {
        match self.lighting_mode {
            LightingMode::NoShading => color,
            LightingMode::FlatShading => {
                let centroid = (t.p1 + t.p2 + t.p3) * (1. / 3.);
                let light_dir = (self.light - centroid).normalized();
                let light_mag = light_dir.dot(t.normal()).max(0.);
                let (r, g, b) = color;
                (
                    (r as f64 * light_mag) as u8,
                    (g as f64 * light_mag) as u8,
                    (b as f64 * light_mag) as u8
                )
            },
        }
    }

    fn fill_bottom_flat_triangle(&mut self, t: Triangle) {
        let (top, mut left, mut right) = t.to_tuple();
        if left.x > right.x { mem::swap(&mut left, &mut right) }

        for y in top.y as PixCoord .. left.y as PixCoord {
            let t = (y - top.y as PixCoord) as Coord / (left.y - top.y);

            let z_left  = top.z + t * (left.z  - top.z);
            let z_right = top.z + t * (right.z - top.z);

            self.texture.set_row(
                (top.x + (left.x  - top.x) * t) as PixCoord,
                (top.x + (right.x - top.x) * t) as PixCoord,
                y,
                z_left,
                z_right,
                self.color
            );
        }
    }

    fn fill_top_flat_triangle(&mut self, t: Triangle) {
        let (mut left, mut right, bot) = t.to_tuple();
        if left.x > right.x { mem::swap(&mut left, &mut right) }

        for y in left.y as PixCoord .. bot.y as PixCoord + 1 {
            let t       = (y - left.y as PixCoord) as Coord / (bot.y - left.y);
            let z_left  = left.z  + t * (bot.z - left.z);
            let z_right = right.z + t * (bot.z - right.z);

            self.texture.set_row(
                (left.x  + (bot.x - left.x)  * t) as PixCoord,
                (right.x + (bot.x - right.x) * t) as PixCoord,
                y,
                z_left,
                z_right,
                self.color
            );
        }
    }

    pub fn clear(&mut self) {
        self.texture.clear();
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
    pub fn set_light_pos(&mut self, pos: Point) { self.light = pos; }
    pub fn set_lighting_mode(&mut self, lighting_mode: LightingMode) {
        self.lighting_mode = lighting_mode;
    }
}
