use super::Rc;

#[test]
fn test_bad() {
    let value = String::from("abcdefg");
    let rc = Rc::new(value);
    let rc1 = Rc::clone(&rc);
    let rc2 = Rc::clone(&rc1);
    let rc3 = Rc::clone(&rc2);
    let rc4 = Rc::clone(&rc3);

    assert_eq!(*rc4, "abcdefg");
}
