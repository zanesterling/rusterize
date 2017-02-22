use sdl2;

use std::error;

use texture::Texture;


pub trait Screen {
    fn display_texture(&mut self, texture: &Texture);

    fn width (&self) -> u32;
    fn height(&self) -> u32;
}


pub struct TextScreen {
    w: u32,
    h: u32,
}

impl TextScreen {
    pub fn new(_: &str, w: u32, h: u32) -> TextScreen {
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

    fn width (&self) -> u32 { self.w }
    fn height(&self) -> u32 { self.h }
}


#[allow(dead_code)]
pub struct GraphicalScreen<'a> {
    w: u32,
    h: u32,
    sdl_renderer: sdl2::render::Renderer<'a>,
}

#[allow(dead_code)]
impl<'a> GraphicalScreen<'a> {
    pub fn new(name: &str, w: u32, h: u32)
        -> Result<GraphicalScreen<'a>, Box<error::Error>>
    {
        // Make an sdl2 window and get the renderer.
        let sdl_context = sdl2::init()?;
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
        // TODO: Draw pixels to renderer.
        self.sdl_renderer.present();
    }

    fn width (&self) -> u32 { self.w }
    fn height(&self) -> u32 { self.h }
}
