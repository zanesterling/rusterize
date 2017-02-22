#![allow(dead_code)]

pub const BLACK: Pixel = (0x00, 0x00, 0x00);
pub const WHITE: Pixel = (0xff, 0xff, 0xff);
pub const RED:   Pixel = (0xff, 0x00, 0x00);
pub const GREEN: Pixel = (0x00, 0xff, 0x00);
pub const BLUE:  Pixel = (0x00, 0x00, 0xff);

pub type Pixel = (u8, u8, u8); // RGB

pub fn as_char(p: Pixel) -> char {
    // FIXME: Improve value formula.
    let (r, g, b) = p;
    let value = r as u16 + g as u16 + b as u16;

    if      value > 0x240 { 'X' }
    else if value > 0x180 { 'x' }
    else if value > 0x0c0 { '.' }
    else                  { ' ' }
}
