use pixel;
use screen::Screen;
use texture::Texture;


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

    pub fn draw_line(&mut self, x1: u32, y1: u32, x2: u32, y2: u32) {
        if !(x2 > x1 && y2 > y1) { panic!("not in the first octant"); }

        let dx: i64 = x2 as i64 - x1 as i64;
        let dy: i64 = y2 as i64 - y1 as i64;

        if dx < dy { panic!("not in the first octant"); }

        let mut error: i64 = 0;
        let mut y = y1;
        for x in x1 .. x2 + 1 {
            error += dy;
            if 2 * error > dx {
                y += 1;
                error -= dx;
            }

            self.texture.set_pixel(x, y, pixel::WHITE);
        }
    }

    pub fn clear(&mut self) {
        self.texture.set_all_pixels(pixel::BLACK);
    }

    pub fn display(&mut self) {
        self.screen.display_texture(&self.texture);
    }
}
