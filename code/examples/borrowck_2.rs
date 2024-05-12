fn main() {
    consume_fn(identity);
}

fn identity(x: &u32) -> &u32 {
    x
}

fn consume_fn<T>(_: impl FnOnce(&u32) -> T) {}
