use crate::Image;

pub mod invert;
pub mod box_blur;
pub mod repeated;

pub trait Filter {
    fn filter(&self, img: &mut Image);
}
