#![feature(try_from)]

#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![deny(clippy::option_unwrap_used)]
#![deny(clippy::result_unwrap_used)]
#![deny(clippy::print_stdout)]

#[macro_use]
extern crate derive_more;

use crate::pixel::Pixel;
use crate::image::Image;

pub mod filters;
pub mod formats;
pub mod pixel;
pub mod image;
