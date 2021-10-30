#[macro_use]
extern crate log;

pub(crate) mod prelude {
    pub use opencv::{
        core::{self as cv_core, prelude::*, Rect, Vector},
        imgcodecs, imgproc,
        prelude::*,
        Error, Result,
    };
}

mod grid;
mod utils;
mod waterfall;

pub(crate) const PAD: i32 = 10;

pub use grid::merge;
pub use waterfall::merge as waterfall;
