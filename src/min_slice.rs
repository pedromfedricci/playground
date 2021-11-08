pub struct MinSlice<T, const N: usize> {
    /// The bounded region of memory. Exactly `N` `T`s.
    pub head: [T; N],
    /// Zero or more remaining `T`s after the `N` in the bounded region.
    pub tail: [T],
}

/// Produce a `&MinSlice` from a slice of `T`s without checking its length.
///
/// # Safety
///
/// The caller is responsible for upholding the length invariant
/// `slice.len() >= N`, in addition to all normal slice invariants.
impl<T, const N: usize> MinSlice<T, N> {
    pub unsafe fn from_slice_unchecked(slice: &[T]) -> &MinSlice<T, N> {
        // This strategy will not check if the new resized slice is in bounds during runtime,
        // so if N > slice.len() this is UB as it will operate over overflowed usize len.
        let resized = core::slice::from_raw_parts(slice.as_ptr(), slice.len() - N);

        &*(resized as *const [T] as *const MinSlice<T, N>)
    }

    pub unsafe fn from_slice_may_panic(slice: &[T]) -> &MinSlice<T, N> {
        if N > slice.len() {
            panic!("MinSlice head's length can't be greater than slice's length");
        }

        // Attempting to use an arithmetic operation that causes an overflow
        // will cause panic during runtime for debug profile.
        let new_len = slice.len() - N;
        // This strategy will check if the provided index is in range
        // during runtime, panicking if it is out of bounds
        // for any profile.
        let resized = &slice[0..new_len];

        &*(resized as *const [T] as *const MinSlice<T, N>)
    }

    pub fn from_slice(slice: &[T]) -> Option<&MinSlice<T, N>> {
        if slice.len() >= N {
            Some(unsafe { Self::from_slice_unchecked(slice) })
        } else {
            None
        }
    }
}

#[test]
fn basic_min_success() {
    let slice = &[1, 2, 3, 4, 5, 6];
    let minslice: &MinSlice<_, 3> = MinSlice::from_slice(slice).unwrap();
    assert_eq!(minslice.tail.len(), 3);
    assert_eq!(minslice.head[0], 1);
    assert_eq!(minslice.tail[2], 6);
}

#[test]
fn basic_min_failure() {
    let slice = &[1, 2, 3, 4, 5, 6];
    let minslice: Option<&MinSlice<_, 7>> = MinSlice::from_slice(slice);
    assert!(minslice.is_none());
}

#[test]
#[should_panic(expected = "MinSlice head's length can't be greater than slice's length")]
fn from_slice_may_panic() {
    let slice = &[1, 2, 3, 4, 5, 6];
    let _minslice: &MinSlice<_, 7> = unsafe { MinSlice::from_slice_may_panic(slice) };
}
