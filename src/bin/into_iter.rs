use libplayground::into_iter::*;

fn main() {
    let mut my_struct = MyStruct(vec!['a', 'b', '3', 'd', '5', '6']);

    // Iterate over an immutable slice iterator.
    for item in (&my_struct).into_iter() {
        println!("{}", item);
    }
    println!("{:?}", my_struct);

    // Iterate over an mutable slice iterator.
    for item in &mut my_struct {
        *item = '?';
    }
    println!("{:?}", my_struct);

    // Also iterate over a mutable slice iterator.
    // NOTE: iter_mut() is implemented by MyStruct, it is not trait boundary,
    // it is a name convention to return Iterators out of structs that can
    // be mutably iterate through.
    let mut count = b'a';
    for item in my_struct.iter_mut() {
        *item = count.into();
        count += 1;
    }

    // Also iterate over an immutable slice iterator.
    // NOTE: iter() is implemented by MyStruct, it is not trait function,
    // it is a name convention to return Iterators out of structs that can
    // be immutably iterate through.
    let _x = my_struct.iter();
    for item in my_struct.iter() {
        println!("{}", item);
    }

    // Iterate over a iterator that moves the value.
    let x = my_struct.into_iter();
    for item in x {
        println!("{}", item)
    }
    // Same as the for loop, consumes self.
    // x.for_each(|item| println!("{}", item));
    //
    // The previous 'for loop' implicitly consumed the value.
    // Next line will be rejected, as the x's value was
    // moved into the 'for loop' to a temporary variable (anonnymous).
    // So now 'x' is logically uninitialized.
    //println!("{:?}", x);
}
