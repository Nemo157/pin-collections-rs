use {
    core::pin::Pin,
    ergo_pin::ergo_pin,
};

existential type PinIterator__Iter<I: PinIterator>: Iterator<Item = I::Item>;
existential type PinIterator__Map<I: PinIterator, F: FnMut(I::Item) -> R, R>: PinIterator<Item = R>;

pub trait PinIterator {
    type Item;

    fn next(self: Pin<&mut Self>) -> Option<Self::Item>;

    fn iter(self) -> PinIterator__Iter<Self>
    where
        Self: Sized + Unpin,
    {
        struct P<I>(I);

        impl<I: PinIterator + Unpin> Iterator for P<I> {
            type Item = I::Item;

            fn next(&mut self) -> Option<Self::Item> {
                Pin::new(&mut self.0).next()
            }
        }

        P(self)
    }

    fn map<F, R>(self, mut f: F) -> PinIterator__Map<Self, F, R>
    where
        Self: Sized,
        F: FnMut(Self::Item) -> R,
    {
        #[ergo_pin]
        gen_iter! {
            for item in pin!(self).iter() {
                yield f(item);
            }
        }
    }
}

pub trait FusedPinIterator: PinIterator {
}

impl<P: PinIterator + ?Sized> PinIterator for Pin<&mut P> {
    type Item = P::Item;

    fn next(self: Pin<&mut Self>) -> Option<Self::Item> {
        Pin::get_mut(self).as_mut().next()
    }
}
