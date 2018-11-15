use {
    core::{pin::Pin, ptr},
    pin_utils::unsafe_unpinned,
};

use crate::{gen_iter, pin_let, PinIterator};

pub struct List<T> {
    head: *mut Node<T>,
}

pub struct Node<T> {
    next: *mut Node<T>,
    attached: bool,
    value: T,
}

impl<T> List<T> {
    unsafe_unpinned!(head: *mut Node<T>);

    pub fn new() -> List<T> {
        List {
            head: ptr::null_mut(),
        }
    }

    pub fn push(mut self: Pin<&mut Self>, mut node: Pin<&mut Node<T>>) {
        node.as_mut().on_attached();
        *node.next() = *self.head();
        *self.head() = node.as_ptr();
    }

    pub fn remove(mut self: Pin<&mut Self>, mut to_remove: Pin<&mut Node<T>>) -> bool {
        if *self.head() == to_remove.as_mut().as_ptr() {
            *self.head() = *to_remove.next();
            to_remove.on_detached();
            return true;
        }

        pin_let!(nodes = self.iter_nodes());
        while let Some(node) = nodes.as_mut().next() {
            if node.next == to_remove.as_mut().as_ptr() {
                unsafe { Pin::get_mut_unchecked(node) }.next = to_remove.next;
                to_remove.on_detached();
                return true;
            }
        }

        return false;
    }

    fn iter_nodes<'a>(
        self: Pin<&'a mut Self>,
    ) -> impl PinIterator<Item = Pin<&'a mut Node<T>>> + 'a {
        gen_iter! {
            let mut node = unsafe { Pin::get_mut_unchecked(self).head };
            while node != ptr::null_mut() {
                yield unsafe { Pin::new_unchecked(&mut *node) };
                node = unsafe { (*node).next };
            }
        }
    }

    pub fn iter<'a>(self: Pin<&'a mut Self>) -> impl PinIterator<Item = &'a mut T> + 'a {
        gen_iter! {
            // pin_let!(nodes = self.iter_nodes());
            // while let Some(mut node) = nodes.as_mut().next() {
            //     yield &mut *node.value();
            // }
            let mut node = unsafe { Pin::get_mut_unchecked(self).head };
            while node != ptr::null_mut() {
                yield unsafe { &mut (*node).value };
                node = unsafe { (*node).next };
            }
        }
    }
}

impl<T> Node<T> {
    unsafe_unpinned!(attached: bool);
    unsafe_unpinned!(next: *mut Self);
    unsafe_unpinned!(value: T);

    pub fn new(value: T) -> Node<T> {
        Node {
            next: ptr::null_mut(),
            attached: false,
            value,
        }
    }

    fn as_ptr(self: Pin<&mut Self>) -> *mut Self {
        unsafe { Pin::get_mut_unchecked(self) as *mut Self }
    }

    fn on_attached(mut self: Pin<&mut Self>) {
        if *self.attached() {
            panic!("node attached while still attached to another list");
        }
        *self.attached() = true;
    }

    fn on_detached(mut self: Pin<&mut Self>) {
        if !*self.attached() {
            panic!("node detached while not attached to a list");
        }
        *self.attached() = false;
    }
}

impl<T> Drop for Node<T> {
    fn drop(&mut self) {
        if self.attached {
            panic!("node dropped while attached to a list");
        }
    }
}
