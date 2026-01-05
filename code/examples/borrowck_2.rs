fn identity(x: &u32) -> &u32 { x }

fn consume_fn<R, F: for<'a> FnOnce(&'a u32) -> R>(_: F) {}

fn main() {
    let fnptr: for<'b> fn(&'b u32) -> &'b u32 = identity;
    consume_fn(fnptr);
}
