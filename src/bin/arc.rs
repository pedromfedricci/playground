use libplayground::arc::Arc;

fn main() {
    let value = String::from("abcdefg");
    let rc = Arc::new(value);
    let rc1 = Arc::clone(&rc);
    let rc2 = Arc::clone(&rc1);
    let rc3 = Arc::clone(&rc2);
    let rc4 = Arc::clone(&rc3);

    assert_eq!(*rc4, "abcdefg");
}
