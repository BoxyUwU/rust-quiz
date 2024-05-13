struct Foo {
    func: fn(),
}

fn print_heheh() {
    println!("ferrisUwu")
}

fn bar(foo: Foo) {
    foo.func();
}

fn main() {
    bar(Foo { func: print_heheh });
}
