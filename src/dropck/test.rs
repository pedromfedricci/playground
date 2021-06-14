use super::*;

#[test]
fn test_sound_generic_drop_with_phantom_data() {
    let mut _world = World {
        inspector: PhantomData,
        _days: Box::new(1),
        _ptr: std::ptr::null(),
    };

    // Won't compile
    //_world._ptr = &Inspector(&world._days);

    // RFC: https://github.com/rust-lang/rfcs/blob/master/text/0769-sound-generic-drop.md
    //
    // For a generic type to soundly implement drop, its generics arguments must strictly outlive it.
    // Here, Since world owns a value of Inspector<'a> (marked by the PhantomData), and
    // Inspector has a lifetime parameter 'a, and Inspector has a Drop impl for 'a, and Inspector can
    // reach what's behind &'a u8, then 'a must strictly outlive the scope of world.
    // The generic lifetime 'a, in this context, is equal to the lifetime of world itself,
    // because it is binded to the value of world.days.
    // Even though field in a struct have a defined destruction order, they do not have
    // a strictly outlive relationship, they all live as long as each other.
    // NOTE: Same goes for values that are binded by a single let statement, like value instantiated in a tuple.
    // Therefore, the previous code won't compile, because the sound generic drop rule (dropck)
    // defines that:
    // - For a generic type to soundly implement drop, its generics arguments must strictly outlive it.
    // That is, in this example, World's generic argument 'a does not STRICTLY outlives
    // an instance of world, 'a is actually just equal to the scope of world itself
    // (eg lives just as long).
}

#[test]
fn test_dropck_binded_lifetime() {
    // Create a struct that ends at the end of this scope.
    let mut _my_struct = MyStruct;

    // Bind a MyBindedStruct instance lifetime
    // to live as long as my_struct, that is, till the
    // end of this scope.
    // The binding ocurrs on bind() execution,
    // because of the lifetime elision.
    let _binded = _my_struct.bind();

    // Won't compile, read bellow.
    //_my_struct.do_it_mut();

    // Since _binded (MyBindedStruct) has a generic lifetime, in this
    // implementation, its lifetime is binded to my_struct (MyStruct) because of
    // the bind() call (lifetime elision ocurrs there, &mut self and '_).
    // In this context, the borrow created on bind() gets "extended"
    // till the moment _binded gets dropped, it got binded to the scope
    // of the _binded instance because _inner (InnerStruct) has a drop impl.
    // Since the borrow was extended, and it was a MUT borrow, you can't
    // have any more borrows (mut or immut) overlapping the initial borrow.
    // Therefore, my_struct here can't call do_it_mut(), or even do_it().
    // NOTE: if bind() were to take a immutable borrow, you could call do_it()
    // here even with the Drop impl fot InnerStruct.
    // To overcame this issue, you could either not impl Drop for MyBindedStruct,
    // or if really needed, you can use the dropck eyepatch feature, which is unsafe,
    // therefore, you must guaranty soundness.
    //
    // Adding the #[may_dangle] attribute makes the type vulnerable to misuse
    // that the borrower checker will not catch, inviting havoc.
    // It is better to avoid adding the attribute.

    // "_binded.drop();"
}

#[test]
fn test_dropck_type_parametric_drop_impl() {
    let mut not_copy = NotCopy(42);
    let ptr: *mut NotCopy = &mut not_copy;
    let _owns = OwnsUnique::new(ptr);
}
