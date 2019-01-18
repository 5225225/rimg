use crate::{Image};
use crate::filters::{FilterParams, box_blur};

pub struct FakeGaussianBlur;

impl FilterParams for FakeGaussianBlur {
    type Params = (usize, usize, usize);

    fn filter_with(&self, img: &mut Image, params: (usize, usize, usize)) {
        let box_blur = box_blur::BoxBlur{};
        for _ in 0..params.0 {
            box_blur.filter_with(img, (params.1, params.2));
        }
    }
}
