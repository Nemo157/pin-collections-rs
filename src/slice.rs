use core::pin::Pin;
use crate::{PinIterator, IntoPinIterator};

existential type SlicePinIterator<'a, T>: PinIterator<Item = Pin<&'a mut T>>;

impl<'a, T> IntoPinIterator for Pin<&'a mut [T]> {
    type Item = Pin<&'a mut T>;
    type IntoIter = SlicePinIterator<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        let mut this = unsafe { Pin::get_unchecked_mut(self) };
            
        gen_iter! {
            loop {
                this = match this.split_first_mut() {
                    Some((first, rest)) => {
                        yield unsafe { Pin::new_unchecked(first) };
                        rest
                    }
                    None => break,
                }
            }
        }
    }
}
