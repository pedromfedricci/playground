#[derive(Debug)]
pub struct MyStruct<T>(pub Vec<T>);

impl<T> IntoIterator for MyStruct<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a, T> IntoIterator for &'a MyStruct<T> {
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut MyStruct<T> {
    type Item = &'a mut T;
    type IntoIter = std::slice::IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter_mut()
    }
}

trait MyFromIterator: Sized {
    fn from_iterator<A, T: Iterator<Item = A>>(iter: T) -> Self;
}

impl<T> MyStruct<T> {
    // Add a 'common' method for structs that can be
    // immutably iterated through.
    //
    // NOTE: NOT taking advantage of lifetime elision,
    // explicitly annotating them.
    #![allow(clippy::needless_lifetimes)]
    pub fn iter<'a>(&'a self) -> std::slice::Iter<'a, T> {
        self.0.iter()
    }

    // Add a 'common' method for structs that can be
    // mutably iterated through.
    //
    // NOTE: Taking advantage of lifetime elision.
    // NOTE: using the 'placeholder lifetime' here is not
    // strictly necessary, but preferred for transparency.
    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, T> {
        self.0.iter_mut()
    }
}
