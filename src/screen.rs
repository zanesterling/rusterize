use sdl2;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::pixels::PixelFormatEnum;

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
    texture: sdl2::render::Texture,
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
        let texture = sdl_renderer
            .create_texture_streaming(PixelFormatEnum::RGB24, w, h)?;

        Ok(GraphicalScreen {
            w: w,
            h: h,
            sdl_renderer: sdl_renderer,
            texture: texture,
        })
    }
}

impl<'a> Screen for GraphicalScreen<'a> {
    fn display_texture(&mut self, texture: &Texture)
        -> Result<(), Box<error::Error>>
    {
        assert!(texture.w == self.w && texture.h == self.h);
        self.texture.with_lock(None, |buffer: &mut [u8], _: usize| {
            for i in 0 .. texture.pixels.len() {
                let pixel = texture.pixels[i].clone();
                buffer[3 * i]     = pixel.r;
                buffer[3 * i + 1] = pixel.g;
                buffer[3 * i + 2] = pixel.b;
            }
        })?;

        self.sdl_renderer.copy(&self.texture, None, None);
        self.sdl_renderer.present();
        Ok(())
    }

    fn width (&self) -> Dimension { self.w }
    fn height(&self) -> Dimension { self.h }
}
