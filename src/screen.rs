use sdl2;
use sdl2::gfx::primitives::DrawRenderer;

use std::error;

use texture::Texture;
use types::*;


pub trait Screen {
    fn display_texture(&mut self, texture: &Texture)
        -> Result<(), Box<error::Error>>;

    fn width (&self) -> Dimension;
    fn height(&self) -> Dimension;
}


#[allow(dead_code)]
pub struct TextScreen {
    w: Dimension,
    h: Dimension,
}

#[allow(dead_code)]
impl TextScreen {
    pub fn new(_: &str, w: Dimension, h: Dimension) -> TextScreen {
        TextScreen {
            w: w,
            h: h,
        }
    }
}

impl Screen for TextScreen {
    fn display_texture(&mut self, texture: &Texture)
        -> Result<(), Box<error::Error>>
    {
        println!("{}", texture);
        Ok(())
    }

    fn width (&self) -> Dimension { self.w }
    fn height(&self) -> Dimension { self.h }
}


#[allow(dead_code)]
pub struct GraphicalScreen<'a> {
    w: Dimension,
    h: Dimension,
    sdl_renderer: sdl2::render::Renderer<'a>,
}

#[allow(dead_code)]
impl<'a> GraphicalScreen<'a> {
    pub fn new(
        name: &str,
        w: Dimension,
        h: Dimension,
        sdl_context: &sdl2::Sdl
    )
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
    fn display_texture(&mut self, texture: &Texture)
        -> Result<(), Box<error::Error>>
    {
        assert!(texture.w == self.w && texture.h == self.h);
        for y in 0..self.h {
            for x in 0..self.w {
                let color = texture.get_pixel(x as Coord, y as Coord);
                self.sdl_renderer.pixel(x as Coord, y as Coord, color);
            }
        }

        self.sdl_renderer.present();
        Ok(())
    }

    fn width (&self) -> Dimension { self.w }
    fn height(&self) -> Dimension { self.h }
}
