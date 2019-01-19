use crate::Image;
use crate::Pixel;
use std::io::{Cursor, Read};
use std::io::Write;
use crate::formats::{Loader, Saver};
use std::convert::TryInto;

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

        for _ in 0..(width * height) as u64 {
            let r = rdr.read_u16::<BE>().unwrap();
            let g = rdr.read_u16::<BE>().unwrap();
            let b = rdr.read_u16::<BE>().unwrap();
            let a = rdr.read_u16::<BE>().unwrap();
            pixels.push(Pixel {
                r: r as f64 / (1 << 16) as f64,
                g: g as f64 / (1 << 16) as f64,
                b: b as f64 / (1 << 16) as f64,
                a: a as f64 / (1 << 16) as f64,
            })
        }

        Image::new(
            width as usize,
            height as usize,
            pixels)
    }
}

fn to_u16(value: f64) -> u16 {
    let shifted = value * (1<<16) as f64;

    (shifted as i64).max(0).min(0xffff) as u16
}

impl Saver for Farbfeld {
    fn save(img: &Image) -> Vec<u8> {
        let mut buf = Vec::new();

        buf.write(b"farbfeld").unwrap();

        buf.write_u32::<BE>(img.width() as u32).unwrap();
        buf.write_u32::<BE>(img.height() as u32).unwrap();

        for pixel in img.pixels() {
            buf.write_u16::<BE>(to_u16(pixel.r)).unwrap();
            buf.write_u16::<BE>(to_u16(pixel.g)).unwrap();
            buf.write_u16::<BE>(to_u16(pixel.b)).unwrap();
            buf.write_u16::<BE>(to_u16(pixel.a)).unwrap();
        }

        buf
    
    }
}
