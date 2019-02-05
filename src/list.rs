use {
    core::{pin::Pin, ptr},
    pin_project::unsafe_project,
    ergo_pin::ergo_pin,
};

use crate::{gen_iter, PinIterator};

#[unsafe_project]
pub struct List<T> {
    head: *mut Node<T>,
}

#[unsafe_project]
pub struct Node<T> {
    next: *mut Node<T>,
    attached: bool,
    value: T,
}

impl<T> List<T> {
    pub fn new() -> List<T> {
        List {
            head: ptr::null_mut(),
        }
    }

    pub fn push(self: Pin<&mut Self>, mut node: Pin<&mut Node<T>>) {
        let this = self.project();
        node.as_mut().on_attached();
        *node.as_mut().project().next = *this.head;
        *this.head = node.as_ptr();
    }

    #[ergo_pin]
    pub fn remove(mut self: Pin<&mut Self>, mut to_remove: Pin<&mut Node<T>>) -> bool {
        let this = self.as_mut().project();
        if *this.head == to_remove.as_mut().as_ptr() {
            *this.head = *to_remove.as_mut().project().next;
            to_remove.on_detached();
            return true;
        }

        for node in pin!(self.iter_nodes()).iter() {
            if node.next == to_remove.as_mut().as_ptr() {
                *node.project().next = *to_remove.as_mut().project().next;
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
            let mut node = unsafe { Pin::get_unchecked_mut(self).head };
            while node != ptr::null_mut() {
                yield unsafe { Pin::new_unchecked(&mut *node) };
                node = unsafe { (*node).next };
            }
        }
    }

    pub fn iter<'a>(self: Pin<&'a mut Self>) -> impl PinIterator<Item = &'a mut T> + 'a {
        #[ergo_pin]
        gen_iter! {
            for node in pin!(self.iter_nodes()).iter() {
                yield node.project().value;
            }
        }
    }
}

impl<T> Node<T> {
    pub fn new(value: T) -> Node<T> {
        Node {
            next: ptr::null_mut(),
            attached: false,
            value,
        }
    }

    fn as_ptr(self: Pin<&mut Self>) -> *mut Self {
        unsafe { Pin::get_unchecked_mut(self) as *mut Self }
    }

    fn on_attached(self: Pin<&mut Self>) {
        let this = self.project();
        if *this.attached {
            panic!("node attached while still attached to another list");
        }
        *this.attached = true;
    }

    fn on_detached(self: Pin<&mut Self>) {
        let this = self.project();
        if !*this.attached {
            panic!("node detached while not attached to a list");
        }
        *this.attached = false;
    }
}

impl<T> Drop for Node<T> {
    fn drop(&mut self) {
        if self.attached {
            panic!("node dropped while attached to a list");
        }
    }
}
