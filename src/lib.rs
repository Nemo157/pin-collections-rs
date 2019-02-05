#![no_std]
#![feature(
    arbitrary_self_types,
    existential_type,
    generators,
    generator_trait,
)]

#[doc(hidden)]
#[macro_use]
pub mod gen_iter;

#[doc(hidden)]
#[macro_use]
mod utils;

pub mod iter;
pub use self::iter::{FusedPinIterator, PinIterator};

pub mod list;
