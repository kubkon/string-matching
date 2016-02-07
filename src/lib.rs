#![crate_name = "pattern_matching"]
#![feature(test)]

extern crate test;

pub use naive::NaiveStringMatcher;
pub use core::StringMatcher;

mod core;
mod naive;

