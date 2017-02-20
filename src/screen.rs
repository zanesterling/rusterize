use std::fmt;
use std::fmt::Display;

use self::pixel::Pixel;


pub struct Screen {
    w: usize,
    h: usize,
    pixels: Vec<Pixel>,
}

impl Screen {
    pub fn new(w: usize, h: usize) -> Screen {
        Screen {
            w: w,
            h: h,
            pixels: vec![Pixel::from_raw(0); w * h],
        }
    }

    pub fn render(&self) {
        println!("{}", self);
    }
}

impl Display for Screen {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "{:-^1$}\n", "", self.w * 2 + 3));
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
        try!(write!(f, "{:-^1$}\n", "", self.w * 2 + 3));
        Ok(())
    }
}


mod pixel {
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
