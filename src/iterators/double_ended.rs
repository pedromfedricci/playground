#[allow(dead_code)]

fn flatten<I>(iter: I) -> Flatten<I::IntoIter>
where
    I: IntoIterator,
    I::Item: IntoIterator,
{
    Flatten::new(iter.into_iter())
}

struct Flatten<I>
where
    I: Iterator,
    I::Item: IntoIterator,
{
    outer: I,
    // inner is a option of a iterator where
    // iterator (IntoIter) is defined by what
    // iterator the outer item can be turned to.
    forward: Option<<I::Item as IntoIterator>::IntoIter>,
    backward: Option<<I::Item as IntoIterator>::IntoIter>,
}

impl<I> Flatten<I>
where
    I: Iterator,
    I::Item: IntoIterator,
{
    fn new(outer: I) -> Flatten<I> {
        Flatten {
            outer,
            forward: None,
            backward: None,
        }
    }
}

impl<I> Iterator for Flatten<I>
where
    I: Iterator,
    I::Item: IntoIterator,
{
    // Item is the item yielded by the iterator
    // that the outer item can be turned to.
    type Item = <I::Item as IntoIterator>::Item;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(ref mut forward) = self.forward {
                if let some_item @ Some(..) = forward.next() {
                    return some_item;
                }
                self.forward = None;
            }

            if let Some(next_forward) = self.outer.next() {
                self.forward = Some(next_forward.into_iter());
            } else {
                return self.backward.as_mut()?.next();
            }
        }
    }
}

impl<I> DoubleEndedIterator for Flatten<I>
where
    I: DoubleEndedIterator,
    I::Item: IntoIterator,
    <I::Item as IntoIterator>::IntoIter: DoubleEndedIterator,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(ref mut inner) = self.backward {
                if let some_item @ Some(..) = inner.next_back() {
                    return some_item;
                }
                self.backward = None;
            }

            if let Some(next_backward) = self.outer.next_back() {
                self.backward = Some(next_backward.into_iter());
            } else {
                return self.forward.as_mut()?.next_back();
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_empty() {
        assert_eq!(flatten(std::iter::empty::<Vec<()>>()).count(), 0);
    }

    #[test]
    fn check_empty_wide() {
        assert_eq!(flatten(vec![Vec::<()>::new(), vec![], vec![]]).count(), 0);
    }

    #[test]
    fn check_one() {
        assert_eq!(flatten(std::iter::once(vec!['a'])).count(), 1);
    }

    #[test]
    fn check_two() {
        assert_eq!(flatten(std::iter::once(vec!['a', 'b'])).count(), 2);
    }

    #[test]
    fn check_two_wide() {
        assert_eq!(flatten(vec![vec!['a'], vec!['b']]).count(), 2);
    }
}
