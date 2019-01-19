use crate::formats::{Loader, Saver};
use crate::Image;
use crate::Pixel;
use std::convert::TryFrom;
use std::io::Write;
use std::io::{Cursor, Read};

use byteorder::{ReadBytesExt, WriteBytesExt, BE};

pub struct Farbfeld;

impl Loader for Farbfeld {
    fn load(bytes: &[u8]) -> Image {
        let mut rdr = Cursor::new(bytes);
        let mut signature: [u8; 8] = [0; 8];
        rdr.read_exact(&mut signature)
            .expect("signature read failed");
        if &signature != b"farbfeld" {
            panic!("Signature did not match");
        }

        let width = rdr.read_u32::<BE>().expect("width read failed");
        let height = rdr.read_u32::<BE>().expect("height read failed");

        let mut pixels = Vec::new();

        for _ in 0..u64::from(width * height) {
            let r = rdr.read_u16::<BE>().expect("truncated file, read failed");
            let g = rdr.read_u16::<BE>().expect("truncated file, read failed");
            let b = rdr.read_u16::<BE>().expect("truncated file, read failed");
            let a = rdr.read_u16::<BE>().expect("truncated file, read failed");
            pixels.push(Pixel {
                r: f64::from(r) / f64::from(1 << 16),
                g: f64::from(g) / f64::from(1 << 16),
                b: f64::from(b) / f64::from(1 << 16),
                a: f64::from(a) / f64::from(1 << 16),
            })
        }

        Image::new(width as usize, height as usize, pixels)
    }
}

// There's not a lot we can do here if our values are out of bounds
#[allow(clippy::cast_possible_truncation)]
#[allow(clippy::cast_sign_loss)]
fn to_u16(value: f64) -> u16 {
    let shifted = value * f64::from(1 << 16);

    (shifted as i64).max(0).min(0xffff) as u16
}

impl Saver for Farbfeld {
    fn save(img: &Image) -> Vec<u8> {
        let mut buf = Vec::new();

        buf.write_all(b"farbfeld").expect("write failed");

        buf.write_u32::<BE>(u32::try_from(img.width()).expect("image width out of bounds"))
            .expect("write failed");
        buf.write_u32::<BE>(u32::try_from(img.height()).expect("image height out of bounds"))
            .expect("write failed");

        for pixel in img.pixels() {
            buf.write_u16::<BE>(to_u16(pixel.r)).expect("write failed");
            buf.write_u16::<BE>(to_u16(pixel.g)).expect("write failed");
            buf.write_u16::<BE>(to_u16(pixel.b)).expect("write failed");
            buf.write_u16::<BE>(to_u16(pixel.a)).expect("write failed");
        }

        buf
    }
}
