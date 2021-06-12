use super::*;

#[test]
fn using_into_as_bound() {
    // Prefer using Into over From when specifying trait bounds
    // on a generic function to ensure that types that only
    // implement Into can be used as well.
    let s = "hello".to_string();
    is_hello_into(s);
}

#[test]
fn using_from_as_bound() {
    // NOT the prefered way.
    // Prior to Rust 1.41, if the destination type was not part of the
    // current crate then you couldn't implement From directly.
    let s = "hello".to_string();
    is_hello_from(s);
}

#[test]
fn using_blanket_impl() {
    // By implementing From<T> for Vec<U>, you can get
    // a Vec<U> from the type T.
    let vec = vec!['a', 'b'];
    let wrap = Wrapper(vec.clone());
    assert_eq!(vec, Vec::from(wrap));

    // By implementing From<T> for Vec<U>, you automatically
    // get a Into<Vec<U>> for T.
    //
    // Need to annotate the type here, compiler can't tell
    // based only on the usage of assert_eq!.
    let wrap_into: Vec<_> = Wrapper(vec.clone()).into();
    assert_eq!(vec, wrap_into);

    // Using the the qualified method call form.
    let wrap = Wrapper(vec.clone());
    assert_eq!(vec, Into::<Vec<_>>::into(wrap));
}
