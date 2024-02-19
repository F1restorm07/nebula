#![no_std]

extern crate alloc; // used for sequence construction

pub mod cursor;
pub mod screen;
pub mod error;

mod colors;
mod attributes;
mod raw;
mod event;
mod input;

pub use crate::{
    colors::*,
    attributes::*,
    raw::*, input::EventIterator,
};

#[macro_export]
macro_rules! csi {
    ($($e:expr),*) => { concat!("\x1b[", $($e),*) };
}
