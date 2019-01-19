use crate::Image;

pub mod box_blur;
pub mod invert;
pub mod repeated;

pub trait Filter {
    fn filter(&self, img: &mut Image);
}
