use super::*;

#[test]
fn test_dropck_binded_lifetime() {
    // Create a struct that ends at the end of this scope.
    let mut my_struct = MyStruct;

    // Bind a MyBindedStruct instance lifetime
    // to live as long as my_struct, that is, till the
    // end of this scope.
    // The binding ocurrs on bind() execution,
    // because of the lifetime elision.
    let _binded = my_struct.bind();

    //my_struct.do_it();

    // Won't compile, read bellow.
    //my_struct.dot_it_mut();

    // Both my_struct and _binded are dropped here at the end of the scope, at the same time.
    // Since _binded (MyBindedStruct) has a generic lifetime, in this
    // context, its lifetime is binded to my_struct (MyStruct) because of
    // the bind() call (lifetime elision ocurrs there, &mut self and '_).
    // Currently, for generic types to soundly implement drop,
    // its generics arguments must STRICTLY outlive it. But in this
    // context, my_struct does not STRICTLY outlives _binded, as both
    // lifetimes end at the same "time" (scope).
    // In this context, the borrow created on bind() gets "extended"
    // till the end of the scope, BECAUSE it got binded to the lifetime
    // of the _binded instance that has a drop impl.
    // Since the borrow was extended, and it was a mut borrow, you can't
    // have any more borrows (mut or immut) overlapping the initial borrow.
    // Therefore, my_struct here can't call do_it_mut()
    // To overcame this issue, you could either not impl Drop for MyBindedStruct,
    // or if really needed, you can use the dropck eyepatch feature, which is unsafe,
    // therefore, you must guaranty soundness.
    //
    // Short: The drop checker forces all borrowed data in a value to STRICTLY outlive that value.
    //
    // Short: Adding the #[may_dangle] attribute makes the type vulnerable to misuse
    // that the borrower checker will not catch, inviting havoc.
    // It is better to avoid adding the attribute.

    // RFC: https://github.com/rust-lang/rfcs/blob/master/text/0769-sound-generic-drop.md
}

#[test]
fn test_dropck_type_parametric_drop_impl() {
    let mut not_copy = NotCopy(42);
    let ptr: *mut NotCopy = &mut not_copy;
    let _owns = OwnsUnique::new(ptr);
}
