fn fn_once_example() {
    let move_val = String::from("This String will be captured by move.");

    // This closure type implements FnOnce, so it will move captured values.
    // It is FnOnce because uses 'mov_val', that is not Copy,
    // by assigning the variable's value to a local variable,
    // effectively moving it.
    let fn_once = || {
        let moved_val = move_val;
        println!("{}", moved_val);
    };

    // Since 'move_val' value was moved into the closure,
    // the variable 'move_val' is now logically uninitialized.
    // The compiler will reject these next lines, as a consequence of that.
    // let move_val = move_val;
    // let borrow_val = &move_val;

    fn_once();
    // fn_once call will also move itself, so calling fn_once() again will not be possible.
    // This next line will be rejected by the compiler.
    // fn_once();
}

fn fn_mut_example() {
    let mut mut_val = String::from("This String will be captured by mutable borrow.");

    // fn_mut changes the value of a captured variable,
    // so not only the variable should be mutable
    // but also the closure variable should be mutable as well.
    //
    // This closure implicitly borrows a mut reference from 'mut_val'
    // so that push_str can modify the value.
    let mut fn_mut = || {
        mut_val.push_str(" This String has been mutated inside the closure.");
    };

    // Cannot mutably borrow more than once at the time.
    // Closure implicitly captured 'mut_val' as a mutable reference.
    // The compiler will reject this next line.
    // let try_borrow_mut = &mut mut_val;

    // Cannot immutably borrow here because closure already mutably borrowed 'mut_val'.
    // The compiler will reject this next line.
    // let try_borrow = &mut_val;

    // This closure type implements FnMut,
    // which means no other reference of captured variables can exists at the same time.
    // FnMut closures do not move any value of captured variables,
    // so they can be called more than once.
    fn_mut();
    fn_mut();

    // Once the usage frame of fn_mut() is completed,
    // other references can refer to the variable.
    let borrow_mut = &mut mut_val;
    *borrow_mut = String::from("This String has been mutated by mut ref after closure usage frame");
    println!("{}", borrow_mut);
    let borrow_immut = &mut_val;
    println!("Repeating: \n\t{}\nFrom shared ref.", borrow_immut);
}

fn fn_example() {
    // Note: mutable because of commented example on line 90.
    let mut val = String::from("This String will be captured by immutable borrow.");

    // Since the captured variable usage inside the closure only
    // needs a shared reference to for the as_str() method,
    // the closure implicitly only makes a immutable borrow.
    let fn_ = || {
        println!("Printing String inside closure: \n\t{}", val.as_str());
    };

    // Can immutably borrow here because closure only borrowed 'val' immutably.
    let borrow_val = &val;
    let borrow_val_2 = &val;
    println!(
        "Can still immutably borrow String inside closure usage frame: \n\t{}",
        borrow_val
    );
    println!(
        "Can still immutably borrow String inside closure usage frame: \n\t{}",
        borrow_val_2
    );

    // Cannot mutably borrow here because an immutable borrow from 'val'
    // is already in place by the closure usage frame.
    // This next line will be rejected by the compiler.
    // val.push_str(" cannot be mutated here");

    // This closure type implements Fn,
    // which means only shared references of captured variables can exists at the same time.
    // Fn closures do not move any value of captured variables,
    // so they can be called more than once.
    fn_();
    fn_();

    // Once the usage frame of fn_() is completed,
    // mutable references can refer to the variable.
    let borrow_mut = &mut val;
    borrow_mut.push_str("This String has been mutated after closure usage frame.");
    println!("{}", borrow_mut);
    let borrow_immut = &val;
    println!("Repeating: \n\t{}\nFrom shared ref.", borrow_immut);
}

fn main() {
    fn_once_example();
    fn_mut_example();
    fn_example();
}
