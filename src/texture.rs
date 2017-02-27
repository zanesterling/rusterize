use std::f64;
use std::fmt;
use std::fmt::Display;

use pixel;
use pixel::Pixel;
use types::*;
use utils::*;


pub struct Texture {
    pub w: Dimension,
    pub h: Dimension,
    pub pixels: Vec<Pixel>,
    z_buffer:   Vec<Coord>,
}

impl Texture {
    pub fn new(w: Dimension, h: Dimension) -> Texture {
        let num_pixels = w as usize * h as usize;
        Texture {
            w: w,
            h: h,
            pixels:   vec![pixel::BLACK;  num_pixels],
            z_buffer: vec![f64::INFINITY; num_pixels],
        }
    }

    pub fn set_pixel(
        &mut self,
        x: PixCoord,
        y: PixCoord,
        z: Coord,
        color: Pixel
    ) {
        if x < 0 || y < 0 { return }
        if self.w < x as Dimension || self.h < y as Dimension { return }
        self.set_pixel_nocheck(x, y, z, color)
    }

    pub fn set_pixel_nocheck(
        &mut self,
        x: PixCoord,
        y: PixCoord,
        z: Coord,
        color: Pixel
    ) {
        let index = y as usize * self.w as usize + x as usize;
        if z >= self.z_buffer[index] { return }
        self.z_buffer[index] = z;
        self.pixels[index]   = color;
    }

    pub fn set_row(
        &mut self,
        x1: PixCoord,
        x2: PixCoord,
        y:  PixCoord,
        z1: Coord,
        z2: Coord,
        color: Pixel
    ) {
        if y  < 0 || y  as Dimension >= self.h { return }
        if x2 < 0 || x1 as Dimension >= self.w { return }

        let start = clamp(x1, 0, (self.w - 1) as PixCoord);
        let end   = clamp(x2, 0, (self.w - 1) as PixCoord);
        let y     = y;

        if x2 <= x1 { return }
        for x in start .. end + 1 {
            let t = ((x - x1) as f64) / ((x2 - x1) as f64);
            let z = z1 * (1. - t) + z2 * t;
            self.set_pixel_nocheck(x, y, z, color);
        }
    }

    pub fn set_all_pixels(&mut self, color: Pixel) {
        for i in 0..self.pixels.len() {
            self.pixels[i] = color;
        }
    }

    pub fn clear(&mut self) {
        for i in 0 .. self.pixels.len() {
            self.pixels[i]   = pixel::BLACK;
            self.z_buffer[i] = f64::INFINITY;
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
