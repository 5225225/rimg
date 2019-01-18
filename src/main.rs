use std::fs::File;
use rimg::formats::{Loader, Saver, farbfeld};
use rimg::filters::{Filter, FilterParams, invert, box_blur, fake_gaussian_blur};
use std::io::{Read, Write};

fn main() {
    let mut in_f = File::open("image.ff").unwrap();
    let mut buf = Vec::new();
    in_f.read_to_end(&mut buf).unwrap();
    let mut img = farbfeld::Farbfeld::load(&buf);

    let gb = fake_gaussian_blur::FakeGaussianBlur{};
    gb.filter_with(&mut img, (3, 100, 100));

    let mut out_f = File::create("image2.ff").unwrap();
    let out_buf = farbfeld::Farbfeld::save(&img);
    out_f.write_all(&out_buf);
}
