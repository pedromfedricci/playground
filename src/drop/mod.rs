#[derive(Debug)]
pub struct NotCopy(i32);

use std::alloc::{Allocator, Global, Layout};
use std::ptr::{drop_in_place, NonNull, Unique};

#[cfg(test)]
mod test;

struct MyBox<T> {
    ptr: Unique<T>,
}

impl<T> Drop for MyBox<T> {
    fn drop(&mut self) {
        unsafe {
            // Unique does not implement Drop
            // so this is basically a no-op.
            drop_in_place(self.ptr.as_ptr());

            let c: NonNull<T> = self.ptr.into();
            // deallocation happens here.
            Global.deallocate(c.cast(), Layout::new::<T>())
        }
    }
}

// This is a no no, as the drop impl for this type
// will try to deallocate the value behind the unique pointer on MyBox,
// but MyBox also has a drop impl that does the same, therefore, there is
// a double free.
struct SuperBox<T> {
    my_box: MyBox<T>,
}

impl<T> Drop for SuperBox<T> {
    fn drop(&mut self) {
        unsafe {
            // Hyper-optimized: deallocate the box's contents for it
            // without `drop`ing the contents
            let c: NonNull<T> = self.my_box.ptr.into();
            Global.deallocate(c.cast::<u8>(), Layout::new::<T>());
        }
    }
}
