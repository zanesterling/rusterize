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
