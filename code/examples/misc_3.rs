// ANCHOR:matters
struct Foo {
    func: fn(),
}

fn bar(foo: Foo) {
    foo.func();
}
// ANCHOR_END:matters

fn main() {}
