use crate::{Image, Pixel};
use crate::filters::FilterParams;

pub struct BoxBlur;

impl FilterParams for BoxBlur {
    type Params = (usize, usize);
    fn filter_with(&self, img: &mut Image, params: (usize, usize)) {
        for row in 0..img.height {
            blur_row(img, row, params.0);
        }
        for col in 0..img.width {
            blur_col(img, col, params.1);
        }
    }
}

fn blur_row(img: &mut Image, row: usize, width: usize) {
    let mut acc = Pixel::default();
    let mut amount = 0;

    let mut scratch = Vec::new();

    for col in 0..width {
        acc += *img.get(col, row).expect("width must be less than image width");
        amount += 1;
    }

    for col in 0..img.width {
        if let Some(head) = img.get(col+width, row) {
            acc += *head;
            amount += 1;
        }

        if (col >= width) {
            let tail = img.get(col-width, row).unwrap();
            acc -= *tail;
            amount -= 1;
        }

        scratch.push(acc / amount as f64);
    }

    for col in 0..img.width {
        *img.get_mut(col, row).unwrap() = scratch[col]
    }
}

fn blur_col(img: &mut Image, col: usize, height: usize) {
    let mut acc = Pixel::default();
    let mut amount = 0;

    let mut scratch = Vec::new();

    for row in 0..height {
        acc += *img.get(col, row).expect("height must be less than image height");
        amount += 1;
    }

    for row in 0..img.height {
        if let Some(head) = img.get(col, row+height) {
            acc += *head;
            amount += 1;
        }

        if (row >= height) {
            let tail = img.get(col, row-height).unwrap();
            acc -= *tail;
            amount -= 1;
        }

        scratch.push(acc / amount as f64);
    }

    for row in 0..img.height {
        *img.get_mut(col, row).unwrap() = scratch[row]
    }
}
