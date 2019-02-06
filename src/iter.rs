use {
    core::pin::Pin,
    ergo_pin::ergo_pin,
};

existential type PinIterator__Iter<I: PinIterator>: Iterator<Item = I::Item>;

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
}

pub trait FusedPinIterator: PinIterator {
}

pub trait IntoPinIterator {
    type Item;
    type IntoIter: PinIterator<Item = Self::Item>;

    fn into_iter(self) -> Self::IntoIter;
}

impl<P: PinIterator> IntoPinIterator for P {
    type Item = <Self as PinIterator>::Item;
    type IntoIter = Self;

    fn into_iter(self) -> Self::IntoIter {
        self
    }
}

impl<P: PinIterator + ?Sized> PinIterator for Pin<&mut P> {
    type Item = P::Item;

    fn next(self: Pin<&mut Self>) -> Option<Self::Item> {
        Pin::get_mut(self).as_mut().next()
    }
}

// TODO: This should be a provided method on `PinIterator`, but existential
// types + closures don't mix well currently.
pub fn map<I, F, R>(iter: I, mut f: F) -> impl PinIterator<Item = R>
where
    I: PinIterator,
    F: FnMut(I::Item) -> R,
{
    #[ergo_pin]
    gen_iter! {
        for item in pin!(iter).iter() {
            yield f(item);
        }
    }
}
