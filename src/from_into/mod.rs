#[cfg(test)]
mod test;

// Using Into as a trait bound.
pub fn is_hello_into<T: Into<Vec<u8>>>(s: T) {
    let bytes = b"hello".to_vec();
    assert_eq!(bytes, s.into());
}

// Using From as trait bound.
pub fn is_hello_from<T>(s: T)
where
    Vec<u8>: From<T>,
{
    let bytes = b"hello".to_vec();
    assert_eq!(bytes, Vec::<u8>::from(s));
}

#[derive(Debug, Clone)]
struct Wrapper<T>(Vec<T>);

// From implementation automatically provides
// an implementation of Into because the std library
// already provides a blanket implementation:
// https://doc.rust-lang.org/stable/std/convert/trait.Into.html#impl-Into%3CU%3E
impl<T> From<Wrapper<T>> for Vec<T> {
    fn from(w: Wrapper<T>) -> Self {
        w.0
    }
}
