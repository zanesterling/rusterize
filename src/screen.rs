use sdl2;
use sdl2::gfx::primitives::DrawRenderer;

use std::error;

use texture::Texture;
use types::Coord;


pub trait Screen {
    fn display_texture(&mut self, texture: &Texture);

    fn width (&self) -> Coord;
    fn height(&self) -> Coord;
}


pub struct TextScreen {
    w: Coord,
    h: Coord,
}

impl TextScreen {
    pub fn new(_: &str, w: Coord, h: Coord) -> TextScreen {
        TextScreen {
            w: w,
            h: h,
        }
    }
}

impl Screen for TextScreen {
    fn display_texture(&mut self, texture: &Texture) {
        println!("{}", texture);
    }

    fn width (&self) -> Coord { self.w }
    fn height(&self) -> Coord { self.h }
}


#[allow(dead_code)]
pub struct GraphicalScreen<'a> {
    w: Coord,
    h: Coord,
    sdl_renderer: sdl2::render::Renderer<'a>,
}

#[allow(dead_code)]
impl<'a> GraphicalScreen<'a> {
    pub fn new(name: &str, w: Coord, h: Coord, sdl_context: &sdl2::Sdl)
        -> Result<GraphicalScreen<'a>, Box<error::Error>>
    {
        // Make an sdl2 window and get the renderer.
        let video_subsystem = sdl_context.video()?;
        let window = video_subsystem
            .window(name, w, h)
            .position_centered()
            .opengl()
            .build()?;
        let sdl_renderer = window.renderer().build()?;

        Ok(GraphicalScreen {
            w: w,
            h: h,
            sdl_renderer: sdl_renderer,
        })
    }
}

impl<'a> Screen for GraphicalScreen<'a> {
    fn display_texture(&mut self, texture: &Texture) {
        assert!(texture.w == self.w && texture.h == self.h);
        for y in 0..self.h {
            for x in 0..self.w {
                let color = texture.get_pixel(x, y);
                self.sdl_renderer.pixel(x as i16, y as i16, color);
            }
        }

        self.sdl_renderer.present();
    }

    fn width (&self) -> Coord { self.w }
    fn height(&self) -> Coord { self.h }
}
