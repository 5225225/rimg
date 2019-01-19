#![feature(try_from)]
#![warn(clippy::all)]

#[macro_use]
extern crate derive_more;

use crate::pixel::Pixel;
use crate::image::Image;

pub mod filters;
pub mod formats;
pub mod pixel;
pub mod image;
