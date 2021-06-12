use std::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct MyBox<T> {
    ptr: *mut T,
}

impl<T> MyBox<T> {
    pub fn new(value: T) -> Self {
        // The Box is consumed, the heap allocation lives
        // and it's pointed by the returing pointer of into_raw().
        // Therefore, the user gets the responsability of releasing
        // the memory, else IT IS A MEMORY LEAK.
        // The heap allocation is released by the drop() implemantation
        // for the type MyBox.
        MyBox {
            ptr: Box::into_raw(Box::new(value)),
        }
    }
}

impl<T> Drop for MyBox<T> {
    fn drop(&mut self) {
        // Safety: ptr was constructed from a Box in the new() method,
        // and has not been freed otherwise since self still exists.
        // (otherwise drop() could not be called)
        unsafe {
            Box::from_raw(self.ptr);
        }
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        // Safety: is valid since it was constructed from a valid T, and
        // turned into a pointer through Box which creates aligned pointers,
        // and hasn't been freed, since self is alive.
        unsafe { &*self.ptr }
    }
}

impl<T> DerefMut for MyBox<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        // Safety: is valid since it was constructed from a valid T, and
        // turned into a pointer through Box which creates aligned pointers,
        // and hasn't been freed, since self is alive.
        // Also, since we have a &mut self, no other mutable reference has been
        // given out of ptr.
        unsafe { &mut *self.ptr }
    }
}

fn main() {
    let value = 42;
    let my_box = MyBox::new(value);
    println!("Box inner: {:?}", *my_box);
}
