use super::*;

#[test]
fn test_my_box_stack_value() {
    let mut not_copy = NotCopy(42);

    let _my_box = MyBox {
        ptr: Unique::new(&mut not_copy).unwrap(),
    };
}

#[test]
fn test_my_super_box_stack_value() {
    let mut not_copy = NotCopy;

    let my_box = MyBox {
        ptr: Unique::new(&mut not_copy).unwrap(),
    };

    let _my_super_box = SuperBox { my_box };
}

#[test]
fn test_my_box_heap_value() {
    let heap_ref = Box::leak(Box::new(NotCopy(42)));

    let _my_box = MyBox {
        ptr: Unique::new(heap_ref).unwrap(),
    };
}

#[test]
fn test_my_super_box_heap_value() {
    // this should try to double free???

    let heap_ref = Box::leak(Box::new(NotCopy(42)));

    let my_box = MyBox {
        ptr: Unique::new(heap_ref).unwrap(),
    };

    let _my_super_box = SuperBox { my_box };

    // it did try to double free!
}
