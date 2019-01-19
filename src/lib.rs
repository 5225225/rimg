#![feature(try_from)]
#![deny(clippy::all)]
#![deny(clippy::cast_possible_truncation)]


#[macro_use]
extern crate derive_more;

use crate::pixel::Pixel;
use crate::image::Image;

pub mod filters;
pub mod formats;
pub mod pixel;
pub mod image;
