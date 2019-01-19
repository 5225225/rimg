use crate::filters::Filter;
use crate::Image;

pub struct Invert;

impl Filter for Invert {
    fn filter(&self, img: &mut Image) {
        for y in 0..img.height() {
            for x in 0..img.width() {
                let mut pix = &mut img[(x, y)];
                pix.r = 1_f64 - pix.r;
                pix.g = 1_f64 - pix.g;
                pix.b = 1_f64 - pix.b;
            }
        }
    }
}
