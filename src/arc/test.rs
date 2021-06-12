use std::{
    sync::atomic::{AtomicUsize, Ordering},
    thread,
};

use super::*;

#[test]
fn sync_shared_counter() {
    let init_val = 5;
    let adds = 10;
    let mut handles = Vec::with_capacity(adds);
    let val = Arc::new(AtomicUsize::new(init_val));

    for _ in 0..adds {
        let val = Arc::clone(&val);

        handles.push(thread::spawn(move || {
            let old_val = val.fetch_add(1, Ordering::SeqCst);
            println!("Previous value: {}", old_val);
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    assert_eq!(init_val + adds, val.load(Ordering::SeqCst));
}
