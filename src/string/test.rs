use std::ops::Deref;

#[test]
fn deref_string() {
    let s = String::from("a");
    // The '*' operator dereferences String to str.
    // Since str is a DST, it needs a reference on the stack poiting to it.
    let str_ref_1 = &*s;
    assert_eq!("a", str_ref_1);

    // *s and *Deref::deref(&s) are equivalent.
    // Again, since both return str (DST), it must be pointed by
    // a reference on the stack.
    let str_ref_2 = &*Deref::deref(&s);
    assert_eq!("a", str_ref_2);

    assert_eq!(str_ref_1, str_ref_2);
}
