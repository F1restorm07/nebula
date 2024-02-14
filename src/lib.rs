#![no_std]

extern crate alloc;

pub mod colors;
pub mod attributes;

#[macro_export]
macro_rules! csi {
    ($($e:expr),*) => { concat!("\x1b[", $($e),*) };
}
