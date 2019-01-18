use crate::Image;

pub mod farbfeld;

pub trait Loader {
    fn load(bytes: &[u8]) -> Image;
}

pub trait Saver {
    fn save(img: &Image) -> Vec<u8>;
}
