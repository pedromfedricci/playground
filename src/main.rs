use std::sync::{Arc, Mutex};
use std::thread;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}

use std::ops::Deref;

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

fn distance<T>(left: *const T, right: *const T) -> isize {
    println!("LEFT as isize: {}", left as isize);
    println!("RIGHT as isize: {}", right as isize);
    let size_of_t = std::mem::size_of::<T>() as isize;
    println!("SIZE OF T: {}", size_of_t);
    (left as isize - right as isize) / size_of_t as isize
}

fn main() {
    let mutex = Arc::new(Mutex::new(0));
    let c_mutex = Arc::clone(&mutex);

    thread::spawn(move || {
        *c_mutex.lock().unwrap() = 10;
    })
    .join()
    .expect("thread::spawn failed");
    assert_eq!(*mutex.lock().unwrap(), 10);
    vec![1];

    let m = MyBox::new(String::from("Rust"));
    hello(m.deref().deref());
    hello(&(*m)[..]);

    let trucks = vec!["garbage truck", "dump truck", "moonstruck"];
    for i in 0..=1 {
        distance(&trucks[i], &trucks[i + 1]);
    }

    let pointer_size = std::mem::size_of::<&u8>();
    println!("{}", pointer_size);

    let pointer_size = std::mem::size_of::<i32>();
    println!("{}", pointer_size);

    let pointer_size = std::mem::size_of::<i64>();
    println!("{}", pointer_size);
}
