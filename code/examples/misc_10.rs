use std::future::Future;

struct ThreadSafePtr<T>(pub *const T);
unsafe impl<T> Send for ThreadSafePtr<T> {}

fn requires_send(_f: impl Future + Send) {}

fn main() {
    let x = 10;
    let y = ThreadSafePtr(&raw const x);

    requires_send(async move {
        println!("{}", unsafe{*y.0})
    })
}
