use sdl2;

use std::error;
use std::fmt;
use std::fmt::Display;

use pixel;
use pixel::Pixel;


pub struct Screen<'a> {
    w: u32,
    h: u32,
    pixels: Vec<Pixel>,

    renderer: sdl2::render::Renderer<'a>,
}

impl<'a> Screen<'a> {
    pub fn new(name: &str, w: u32, h: u32, sdl_context: &sdl2::Sdl)
        -> Result<Screen<'a>, Box<error::Error>>
    {
        // Make an sdl2 window and get the renderer.
        let video_subsystem = sdl_context.video()?;
        let window = video_subsystem
            .window(name, w, h)
            .position_centered()
            .opengl()
            .build()?;
        let renderer = window.renderer().build()?;

        Ok(Screen {
            w: w,
            h: h,
            pixels: vec![pixel::BLACK; (w * h) as usize],
            renderer: renderer,
        })
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, color: Pixel) {
        if self.w < x || self.h < y { return; }
        let index = y as usize * self.w as usize + x as usize;
        self.pixels[index] = color;
    }

    pub fn set_pixel_nocheck(
        &mut self,
        x: u32,
        y: u32,
        color: Pixel
    ) {
        let index = y as usize * self.w as usize + x as usize;
        self.pixels[index] = color;
    }

    pub fn clear(&mut self) {
        for i in 0..self.pixels.len() {
            self.pixels[i] = pixel::BLACK;
        }
    }

    pub fn display(&mut self) {
        // TODO: Draw pixels to renderer.
        self.renderer.present();
    }

    pub fn display_text(&self) {
        println!("{}", self);
    }
}

impl<'a> Display for Screen<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Draw top bar.
        try!(write!(f, "{:-^1$}\n", "", self.w as usize * 2 + 3));

        // Draw rows.
        self.pixels
            .chunks(self.w as usize)
            .map(|row| {
                try!(write!(f, "| "));
                for p in row {
                    try!(write!(f, "{} ", p.as_char()));
                }
                try!(write!(f, "|\n"));
                Ok(())
            }).collect::<Result<Vec<_>, fmt::Error>>()?;

        // Draw bottom bar.
        try!(write!(f, "{:-^1$}\n", "", self.w as usize * 2 + 3));
        Ok(())
    }
}
