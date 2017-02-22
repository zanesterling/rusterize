use std::fmt;
use std::fmt::Display;

use pixel;
use pixel::Pixel;
use types::Coord;


pub struct Texture {
    pub w: Coord,
    pub h: Coord,
    pub pixels: Vec<Pixel>,
}

impl Texture {
    pub fn new(w: Coord, h: Coord) -> Texture {
        Texture {
            w: w,
            h: h,
            pixels: vec![pixel::BLACK; w as usize * h as usize],
        }
    }

    pub fn get_pixel(&self, x: Coord, y: Coord) -> Pixel {
        self.pixels[x as usize + y as usize * self.w as usize].clone()
    }

    pub fn set_pixel(&mut self, x: Coord, y: Coord, color: Pixel) {
        if self.w < x || self.h < y { return; }
        let index = y as usize * self.w as usize + x as usize;
        self.pixels[index] = color;
    }

    pub fn set_pixel_nocheck(
        &mut self,
        x: Coord,
        y: Coord,
        color: Pixel
    ) {
        let index = y as usize * self.w as usize + x as usize;
        self.pixels[index] = color;
    }

    pub fn set_all_pixels(&mut self, color: Pixel) {
        for i in 0..self.pixels.len() {
            self.pixels[i] = color.clone();
        }
    }
}

impl Display for Texture {
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
