#![no_std]

extern crate alloc;

pub mod colors;
pub mod attributes;
pub mod cursor;
pub mod screen;
pub mod raw;
pub mod event;
pub mod input;
pub mod error;

#[macro_export]
macro_rules! csi {
    ($($e:expr),*) => { concat!("\x1b[", $($e),*) };
}
