use {
    core::{
        ops::{Generator, GeneratorState},
        pin::Pin,
    },
    pin_utils::unsafe_pinned,
};

use crate::PinIterator;

pub struct GenIter<G> {
    gen: G,
}

impl<G: Generator<Return = ()>> GenIter<G> {
    pub fn new(gen: G) -> GenIter<G> {
        GenIter { gen }
    }

    unsafe_pinned!(gen: G);
}

impl<G: Generator<Return = ()>> PinIterator for GenIter<G> {
    type Item = G::Yield;

    fn next(self: Pin<&mut Self>) -> Option<Self::Item> {
        match self.gen().resume() {
            GeneratorState::Yielded(item) => Some(item),
            GeneratorState::Complete(()) => None,
        }
    }
}

#[macro_export]
macro_rules! gen_iter {
    ($($tt:tt)*) => {
        $crate::gen_iter::GenIter::new(static move || {
            #[allow(unreachable_code)] {
                if false { yield return }
            };
            $($tt)*
        })
    }
}
