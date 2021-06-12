use std::{cell::Cell, marker::PhantomData, ops::Deref, ptr::NonNull};

#[cfg(test)]
mod test;

#[derive(Debug)]
struct RcBox<T> {
    value: T,
    strong: Cell<usize>,
}

#[derive(Debug)]
pub struct Rc<T> {
    ptr: NonNull<RcBox<T>>,
    _marker: PhantomData<RcBox<T>>,
}

impl<T> Rc<T> {
    pub fn new(value: T) -> Self {
        let rc_box = Box::new(RcBox {
            value,
            strong: Cell::new(1),
        });

        Self {
            ptr: Box::leak(rc_box).into(),
            //ptr: unsafe { NonNull::new_unchecked(Box::into_raw(rc_box)) },
            //ptr: NonNull::new(Box::into_raw(boxed)).unwrap(),
            _marker: PhantomData,
        }
    }

    unsafe fn _get_mut_unchecked(this: &mut Self) -> &mut T {
        &mut (*this.ptr.as_ptr()).value
    }
}

impl<T> Clone for Rc<T> {
    fn clone(&self) -> Self {
        let inner = unsafe { self.ptr.as_ref() };
        inner.strong.set(inner.strong.get() + 1);

        Self {
            ptr: self.ptr,
            _marker: PhantomData,
        }
    }
}

impl<T> Deref for Rc<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &unsafe { self.ptr.as_ref() }.value
    }
}

impl<T> Drop for Rc<T> {
    fn drop(&mut self) {
        let inner = unsafe { self.ptr.as_ref() };
        inner.strong.set(inner.strong.get() - 1);

        if inner.strong.get() == 0 {
            unsafe {
                Box::from_raw(self.ptr.as_ptr());
            }
        }
    }
}
