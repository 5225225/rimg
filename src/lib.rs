#![feature(try_from)]
#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![deny(clippy::option_unwrap_used)]
#![deny(clippy::result_unwrap_used)]
#![deny(clippy::print_stdout)]

#[macro_use]
extern crate derive_more;

use crate::image::Image;
use crate::pixel::Pixel;

pub mod filters;
pub mod formats;
pub mod image;
pub mod pixel;
