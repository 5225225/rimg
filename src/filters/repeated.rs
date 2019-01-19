use crate::{Image};
use crate::filters::{Filter};

pub struct Repeated<T: Filter> {
    pub iterations: u64,
    pub filter: T,
}

impl<T: Filter> Filter for Repeated<T> {
    fn filter(&self, img: &mut Image) {
        for _ in 0..self.iterations {
            self.filter.filter(img);
        }
    }
}
