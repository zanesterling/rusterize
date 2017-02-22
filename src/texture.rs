use std::fmt;
use std::fmt::Display;

use pixel;
use pixel::Pixel;
use types::*;


pub struct Texture {
    pub w: Dimension,
    pub h: Dimension,
    pub pixels: Vec<Pixel>,
}

impl Texture {
    pub fn new(w: Dimension, h: Dimension) -> Texture {
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
        if x < 0 || y < 0 { return }
        if self.w < x as Dimension || self.h < y as Dimension { return }

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
                    try!(write!(f, "{} ", pixel::as_char(*p)));
                }
                try!(write!(f, "|\n"));
                Ok(())
            }).collect::<Result<Vec<_>, fmt::Error>>()?;

        // Draw bottom bar.
        try!(write!(f, "{:-^1$}\n", "", self.w as usize * 2 + 3));
        Ok(())
    }
}
