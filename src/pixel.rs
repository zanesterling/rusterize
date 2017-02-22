#![allow(dead_code)]

use sdl2::gfx::primitives::ToColor;

pub const BLACK: Pixel = Pixel { r: 0x00, g: 0x00, b: 0x00 };
pub const WHITE: Pixel = Pixel { r: 0xff, g: 0xff, b: 0xff };
pub const RED:   Pixel = Pixel { r: 0xff, g: 0x00, b: 0x00 };
pub const GREEN: Pixel = Pixel { r: 0x00, g: 0xff, b: 0x00 };
pub const BLUE:  Pixel = Pixel { r: 0x00, g: 0x00, b: 0xff };

#[derive(Clone)]
pub struct Pixel {
    pub r: u8,
    pub g: u8,
    pub b: u8,
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

impl ToColor for Pixel {
    fn as_rgba(&self) -> (u8, u8, u8, u8) {
        (self.r, self.g, self.b, 0xff)
    }

    fn as_u32(&self) -> u32 {
        ((self.r as u32) << 0)  |
        ((self.g as u32) << 8)  |
        ((self.b as u32) << 16) |
        0xff000000
    }
}
