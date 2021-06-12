use std::ffi::OsStr;

#[cfg(test)]
mod test;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct MyPath();

impl MyPath {
    // Flexing the Sized bound allows this function
    // to be called with string literals directly:
    // new("/some/path"), where else it would need
    // to be behind a reference.
    pub fn new<S: AsRef<OsStr> + ?Sized>(_s: &S) -> Self {
        Self()
    }
}

pub enum MyCow<'a, B: ?Sized>
where
    B: ToOwned,
{
    Borrowed(&'a B),
    Owned(<B as ToOwned>::Owned),
}
