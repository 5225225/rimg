use rimg::filters::{box_blur, repeated, Filter};
use rimg::formats::{farbfeld, Loader, Saver};
use std::fs::File;
use std::io::{Read, Write};

fn main() {
    let mut in_f = File::open("image.ff").unwrap();
    let mut buf = Vec::new();
    in_f.read_to_end(&mut buf).unwrap();
    let mut img = farbfeld::Farbfeld::load(&buf);

    let gb = repeated::Repeated {
        iterations: 3,
        filter: box_blur::BoxBlur {
            width: 10,
            height: 2,
        },
    };

    gb.filter(&mut img);

    let mut out_f = File::create("image2.ff").unwrap();
    let out_buf = farbfeld::Farbfeld::save(&img);
    out_f.write_all(&out_buf).unwrap();
}
