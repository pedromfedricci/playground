use std::marker::PhantomData;

#[cfg(test)]
mod test;
struct InnerStruct<'a> {
    _marker: PhantomData<&'a ()>,
}

// unsafe impl<#[may_dangle] 'a> Drop for InnerStruct<'a> {
//     fn drop(&mut self) {
//         // something that you really need to do on drop
//         // for this struct, that is, the drop impl is necessary.
//         println!("InnerStruct drop call");

//         // Looking at some reference market with lifetime 'a
//         // during drop execution can lead to use after free,
//         // even though InnerStruct supose to live as long as 'a.
//         //
//         // That's because, currently, the compiler doesn't analyse
//         // whether the referenced values were destroyed first or not.
//         // And so, it behaves in a conservative way, assuming that,
//         // during the drop execution, the call will look at those values.
//         // The #[may_dangle] is a way (unsafe) for the user to tell the
//         // compiler that they will not attempt to look at what is behind
//         // that lifetime or value.
//     }
// }

// If the eyepatch is not used #![feature(dropck_eyepatch)],
// then the drop checker will assume that InnerStruct drop impl
// WILL "access" or "see" what's behind the reference it holds
// during the drop execution (even in this particular case where it
// is only a phantom data).
//
impl Drop for InnerStruct<'_> {
    fn drop(&mut self) {
        println!("InnerStruct drop call (no eyepatch)");
    }
}

pub struct MyBindedStruct<'a> {
    _inner: InnerStruct<'a>,
}

pub struct MyStruct;

impl MyStruct {
    pub fn bind(&mut self) -> MyBindedStruct<'_> {
        MyBindedStruct {
            _inner: InnerStruct {
                _marker: PhantomData,
            },
        }
    }

    pub fn do_it(&self) {}
    pub fn dot_it_mut(&mut self) {}
}

// ###########################################################################

pub struct MyUnique<T: ?Sized> {
    pub pointer: *const T,
    // NOTE: this marker has no consequences for variance, but is necessary
    // for dropck to understand that we logically own a `T`.
    //
    // For details, see:
    // https://github.com/rust-lang/rfcs/blob/master/text/0769-sound-generic-drop.md#phantom-data
    _marker: PhantomData<T>,
}

impl<T: ?Sized> MyUnique<T> {
    pub fn new(ptr: *mut T) -> Option<Self> {
        if !ptr.is_null() {
            // SAFETY: The pointer has already been checked and is not null.
            Some(MyUnique {
                pointer: ptr as _,
                _marker: PhantomData,
            })
        } else {
            None
        }
    }
}

pub struct OwnsUnique<T> {
    pub ptr: MyUnique<T>,
}

impl<T> OwnsUnique<T> {
    pub fn new(value: T) -> OwnsUnique<T> {
        let pointer = Box::leak(Box::new(value));
        OwnsUnique {
            ptr: unsafe { MyUnique::new(pointer).unwrap_unchecked() },
        }
    }
}

impl<T> Drop for OwnsUnique<T> {
    fn drop(&mut self) {
        let ptr = self.ptr.pointer;
        unsafe {
            Box::from_raw(ptr as *mut T);
        }
    }
}

#[derive(Debug)]
pub struct NotCopy(i32);
