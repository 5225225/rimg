#![feature(try_from)]

#[macro_use]
extern crate derive_more;

use std::ops::{Index, Deref, DerefMut, IndexMut, Div};

pub mod filters;
pub mod formats;

#[derive(Debug)]
pub struct Image {
    width: usize,
    height: usize,
    pixels: Vec<Pixel>,
}


impl Index<(usize, usize)> for Image {
    type Output = Pixel;

    fn index(&self, coords: (usize, usize)) -> &Pixel {
        let x = coords.0;
        let y = coords.1;
        let pos = y * self.width + x;
        &self.pixels[pos]
    }
}

impl IndexMut<(usize, usize)> for Image {
    fn index_mut(&mut self, coords: (usize, usize)) -> &mut Pixel {
        let x = coords.0;
        let y = coords.1;
        let pos = y * self.width + x;
        &mut self.pixels[pos]
    }
}

impl Image {
    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    fn bounds_check(&self, x: usize, y: usize) -> bool {
        x < self.width && y < self.height
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&Pixel> {
        let pos = y * self.width + x;
        if self.bounds_check(x, y) == false {
            return None;
        }
        self.pixels.get(pos)
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut Pixel> {
        let pos = y * self.width + x;
        if self.bounds_check(x, y) == false {
            return None;
        }
        self.pixels.get_mut(pos)
    }
}

impl Deref for Image {
    type Target = [Pixel];

    fn deref(&self) -> &[Pixel] {
        &self.pixels
    }
}

impl DerefMut for Image {
    fn deref_mut(&mut self) -> &mut [Pixel] {
        &mut self.pixels
    }
}

#[derive(Default, Copy, Clone, Debug, Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign)]
pub struct Pixel {
    r: f64,
    g: f64,
    b: f64,
    a: f64,
}
