trait A<'a> {
    type Assoc;
}

trait B {}

fn f(_: impl for<'a> A<'a, Assoc = impl B + 'a>) {}

impl<'a> A<'a> for () {
    type Assoc = &'a u8;
}

impl B for &u8 {}

fn main() {
    f(());
}
