use sdl2;

use std::error;
use std::fmt;
use std::fmt::Display;

use self::pixel::Pixel;


pub struct Screen<'a> {
    w: usize,
    h: usize,
    pixels: Vec<Pixel>,

    renderer: sdl2::render::Renderer<'a>,
}

impl<'a> Screen<'a> {
    pub fn new(name: &str, w: usize, h: usize, sdl_context: &sdl2::Sdl)
        -> Result<Screen<'a>, Box<error::Error>>
    {
        // Make an sdl2 window and get the renderer.
        let video_subsystem = sdl_context.video()?;
        let window = video_subsystem
            .window(name, w as u32, h as u32)
            .position_centered()
            .opengl()
            .build()?;
        let renderer = window.renderer().build()?;

        Ok(Screen {
            w: w,
            h: h,
            pixels: vec![pixel::BLACK; w * h],
            renderer: renderer,
        })
    }

    pub fn display(&mut self) {
        // TODO: Draw pixels to renderer.
        self.renderer.present();
    }

    pub fn display_text(&self) {
        println!("{}", self);
    }

    pub fn clear(&mut self) {
        for i in 0..self.pixels.len() {
            self.pixels[i] = pixel::BLACK;
        }
    }
}

impl<'a> Display for Screen<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Draw top bar.
        try!(write!(f, "{:-^1$}\n", "", self.w * 2 + 3));

        // Draw rows.
        self.pixels
            .chunks(self.w)
            .map(|row| {
                try!(write!(f, "| "));
                for p in row {
                    try!(write!(f, "{} ", p.as_char()));
                }
                try!(write!(f, "|\n"));
                Ok(())
            }).collect::<Result<Vec<_>, fmt::Error>>()?;

        // Draw bottom bar.
        try!(write!(f, "{:-^1$}\n", "", self.w * 2 + 3));
        Ok(())
    }
}


mod pixel {
    pub const BLACK: Pixel = Pixel { r: 0x00, g: 0x00, b: 0x00 };
    pub const WHITE: Pixel = Pixel { r: 0xff, g: 0xff, b: 0xff };

    #[derive(Clone)]
    pub struct Pixel {
        r: u8,
        g: u8,
        b: u8,
    }

    impl Pixel {
        pub fn from_raw(raw_val: u32) -> Pixel {
            Pixel {
                r: ( raw_val        & 0xff) as u8,
                g: ((raw_val >> 8)  & 0xff) as u8,
                b: ((raw_val >> 16) & 0xff) as u8,
            }
        }

        pub fn as_char(&self) -> char {
            // FIXME: Improve value formula.
            let value = self.r as u16 + self.g as u16 + self.b as u16;

            if      value > 0x240 { 'X' }
            else if value > 0x180 { 'x' }
            else if value > 0x0c0 { '.' }
            else                  { ' ' }
        }
    }
}
