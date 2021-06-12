#[cfg(test)]
mod test;

#[macro_export]
macro_rules! my_assert_eq_match_without_refs {
    ($left:expr, $right:expr $(,)?) => {{
        match ($left, $right) {
            (left_val, right_val) => {
                if !(left_val == right_val) {
                    // The reborrows below are intentional. Without them, the stack slot for the
                    // borrow is initialized even before the values are compared, leading to a
                    // noticeable slow down.
                    panic!(
                        r#"assertion failed: `(left == right)`
  left: `{:?}`,
 right: `{:?}`"#,
                        &left_val, &right_val
                    )
                }
            }
        }
    }};
}

#[macro_export]
macro_rules! my_assert_eq_match_with_refs {
    ($left:expr, $right:expr $(,)?) => {{
        match (&$left, &$right) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    // The reborrows below are intentional. Without them, the stack slot for the
                    // borrow is initialized even before the values are compared, leading to a
                    // noticeable slow down.
                    panic!(
                        r#"assertion failed: `(left == right)`
  left: `{:?}`,
 right: `{:?}`"#,
                        &*left_val, &*right_val
                    )
                }
            }
        }
    }};
}

#[derive(Debug, PartialEq, Eq)]
pub struct MyStruct(Vec<u8>);
