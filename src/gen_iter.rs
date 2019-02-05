use {
    core::{
        ops::{Generator, GeneratorState},
        pin::Pin,
    },
    pin_project::unsafe_project,
};

use crate::{FusedPinIterator, PinIterator};

#[unsafe_project(Unpin)]
pub struct GenIter<G> {
    #[pin]
    gen: Option<G>,
}

impl<G: Generator<Return = ()>> GenIter<G> {
    pub fn new(gen: G) -> GenIter<G> {
        GenIter { gen: Some(gen) }
    }
}

impl<G: Generator<Return = ()>> PinIterator for GenIter<G> {
    type Item = G::Yield;

    fn next(self: Pin<&mut Self>) -> Option<Self::Item> {
        let mut this = self.project();
        match this.gen.as_mut().as_pin_mut().map(|g| g.resume()) {
            Some(GeneratorState::Yielded(item)) => Some(item),
            Some(GeneratorState::Complete(())) => {
                this.gen.set(None);
                None
            },
            None => None,
        }
    }
}

impl<G: Generator<Return = ()>> FusedPinIterator for GenIter<G> {
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
