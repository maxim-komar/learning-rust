// Rust provides asynchronous `channels` for communication between threads.
// Channels allow a undirectional flow of information between two end-points:
// the `Sender` and the `Receiver`

use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::thread;

static NTHREADS: i32 = 3;

fn main() {
    let (tx, rx): (Sender<i32>, Receiver<i32>) = mpsc::channel();
    let mut children = Vec::new();

    for id in 0..NTHREADS {
        // The sender endpoint can be copied
        let thread_tx = tx.clone();

        let child = thread::spawn(move || {
            thread_tx.send(id).unwrap();

            println!("thread {} finished", id);
        });

        children.push(child);
    }

    let mut ids = Vec::with_capacity(NTHREADS as usize);
    for _ in 0..NTHREADS {
        ids.push(rx.recv());
    }

    for child in children {
        child.join().expect("oops! the child thread panicked");
    }

    println!("{:?}", ids);
}
