use std::thread;
use some_lib::channel;

fn main() {
    let (tx, rx) = channel();
    let data = vec![1, 2, 3];

    thread::spawn(move || {
        // `data` now belongs to the closure. It is unreachable from the parent thread.
        data.pop();
        tx.send(data);
        // `send` took ownership of `data`: it is now unreachable.
        // data.push(3) <-- error
    });

    // data.push(5);    <-- error
    let data = rx.recv();
    data.push(42); // ok
}

impl<T> for Channel<T> {
    fn send(&self, t: T);
    fn recv(&self) -> T;
}
