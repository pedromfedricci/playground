pub mod str_split {
    pub struct StrSplit<'a> {
        pub remainder: Option<&'a str>,
        pub delimeter: &'a str,
    }

    impl<'a> StrSplit<'a> {
        pub fn new(haystack: &'a str, delimeter: &'a str) -> Self {
            Self {
                remainder: Some(haystack),
                delimeter,
            }
        }
    }

    impl<'a> Iterator for StrSplit<'a> {
        type Item = &'a str;
        fn next(&mut self) -> Option<Self::Item> {
            let remainder = self.remainder.as_mut()?;
            if let Some(next_delim) = remainder.find(self.delimeter) {
                let until_delim = &remainder[..next_delim];
                *remainder = &remainder[(next_delim + self.delimeter.len())..];
                Some(until_delim)
            } else {
                self.remainder.take()
            }
        }
    }

    fn _until_char<'s>(s: &'s str, c: char) -> &'s str {
        // the problem arises when you try to return the string slice ref from
        // next() to outer scope. Since both StrSplit 'remainder' and 'delimeter'
        // are annotated with the same 'a generic lifetime, 'a will be the shortest
        // concrete lifetime out of both. In this case, 'a will be associated with
        // the concrete lifetime of the String returned by format!(), pointed by
        // the StrSlice instance's 'delimeter', that only lives up to the end of this function scope.
        // Threfore, 'a is shorter than 's, since this function defines that 's outlives
        // the function, and 'a was defined to be valid only inside the function scope.
        // &str returned by next() lives only by whatever concrete lifetime that is associated with 'a,
        // 'a is shorter than 's, and shorter lifetimes can't be extended to longer ones.
        // This implementation does not fullfill the contract defined by the function signature,
        // as you are trying to return a &'a str as a &'s str when 'a is shorter than 's, which is not allowed.
        // The compiler will reject this implementation.
        //
        StrSplit::new(s, &format!("{}", c)).next().unwrap();
        todo!()
    }
}
