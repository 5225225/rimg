use crate::Image;
use crate::Pixel;
use std::io::{Cursor, Read};
use std::convert::TryFrom;
use std::io::Write;
use crate::formats::{Loader, Saver};

use byteorder::{ReadBytesExt, WriteBytesExt, BE};

pub struct Farbfeld;

impl Loader for Farbfeld {
    fn load(bytes: &[u8]) -> Image {
        let mut rdr = Cursor::new(bytes);
        let mut signature: [u8; 8] = [0; 8];
        rdr.read_exact(&mut signature).unwrap();
        if &signature != b"farbfeld" {
            panic!("Signature did not match");
        }

        let width = rdr.read_u32::<BE>().unwrap();
        let height = rdr.read_u32::<BE>().unwrap();

        let mut pixels = Vec::new();

        for _ in 0..u64::from(width * height) {
            let r = rdr.read_u16::<BE>().unwrap();
            let g = rdr.read_u16::<BE>().unwrap();
            let b = rdr.read_u16::<BE>().unwrap();
            let a = rdr.read_u16::<BE>().unwrap();
            pixels.push(Pixel {
                r: f64::from(r) / f64::from(1 << 16),
                g: f64::from(g) / f64::from(1 << 16),
                b: f64::from(b) / f64::from(1 << 16),
                a: f64::from(a) / f64::from(1 << 16),
            })
        }

        Image::new(
            width as usize,
            height as usize,
            pixels)
    }
}

// There's not a lot we can do here if our values are out of bounds
#[allow(clippy::cast_possible_truncation)]
fn to_u16(value: f64) -> u16 {
    let shifted = value * f64::from(1<<16);

    (shifted as i64).max(0).min(0xffff) as u16
}

impl Saver for Farbfeld {
    fn save(img: &Image) -> Vec<u8> {
        let mut buf = Vec::new();

        buf.write_all(b"farbfeld").unwrap();

        buf.write_u32::<BE>(u32::try_from(img.width()).unwrap()).unwrap();
        buf.write_u32::<BE>(u32::try_from(img.height()).unwrap()).unwrap();

        for pixel in img.pixels() {
            buf.write_u16::<BE>(to_u16(pixel.r)).unwrap();
            buf.write_u16::<BE>(to_u16(pixel.g)).unwrap();
            buf.write_u16::<BE>(to_u16(pixel.b)).unwrap();
            buf.write_u16::<BE>(to_u16(pixel.a)).unwrap();
        }

        buf
    
    }
}
