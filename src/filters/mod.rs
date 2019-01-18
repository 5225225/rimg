use crate::Image;

pub mod invert;
pub mod box_blur;
pub mod fake_gaussian_blur;

pub trait Filter {
    fn filter(&self, img: &mut Image);
}

pub trait FilterParams {
    type Params;

    fn filter_with(&self, img: &mut Image, params: Self::Params);
}

impl Filter for FilterParams<Params=()> {
    fn filter(&self, img: &mut Image) {
        self.filter_with(img, ())
    }
}
