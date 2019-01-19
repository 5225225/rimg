#![feature(try_from)]

#[macro_use]
extern crate derive_more;

use std::ops::{Index, Deref, DerefMut, IndexMut, Div};

use crate::pixel::Pixel;
use crate::image::Image;

pub mod filters;
pub mod formats;
pub mod pixel;
pub mod image;
