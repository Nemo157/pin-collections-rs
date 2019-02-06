#![no_std]
#![feature(
    arbitrary_self_types,
    existential_type,
    generators,
    generator_trait,
    proc_macro_hygiene,
)]

#[doc(hidden)]
#[macro_use]
pub mod gen_iter;

pub mod iter;
pub use self::iter::{FusedPinIterator, PinIterator, IntoPinIterator};

pub mod list;

mod slice;
