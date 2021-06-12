use super::MyStruct;
use crate::my_assert_eq_match_with_refs;
use crate::my_assert_eq_match_without_refs;

#[test]
fn test_my_assert_eq() {
    let struct_1 = MyStruct(Vec::new());
    let struct_2 = MyStruct(Vec::new());

    // using `&` in the match statement, borrowing the
    // values from the arguments, therefore no move semantics or copies,
    // even when owned values are passed as parameters.
    my_assert_eq_match_with_refs!(struct_1, struct_2);
    my_assert_eq_match_with_refs!(struct_1, struct_2);
    my_assert_eq_match_with_refs!(&struct_1, &struct_2);
    my_assert_eq_match_with_refs!(&&struct_1, &&struct_2);
    my_assert_eq_match_with_refs!(&&&struct_1, &&&struct_2);

    print!("{:?}", struct_1);
    print!("{:?}", struct_2);

    // not using `&` in the match statement, therefore,
    // if a owned value is passed, it will be moved if it's not Copy,
    // or it will be copied if Copy.
    my_assert_eq_match_without_refs!(struct_1, struct_2);

    // will not work as the values were moved.
    //
    //my_assert_eq_match_without_refs!(struct_1, struct_2);
    //my_assert_eq_match_without_refs!(&struct_1, &struct_2);
}

#[test]
fn test_match() -> Result<(), &'static str> {
    let struct_1 = MyStruct(Vec::new());
    let struct_2 = MyStruct(Vec::new());

    match (&struct_1, struct_2) {
        (s1, s2) => {
            let _ = s1.0.to_ascii_uppercase();
            let _ = s2.0.to_ascii_lowercase();
        }
    }

    // since struct_1 was not moved to the match scope,
    // it's still valid in this scope.
    println!("{:?}", struct_1);

    // struct_2 was moved, MyStruct does not implement Copy.
    //
    //println!("{:?}", struct_2);

    Ok(())
}
