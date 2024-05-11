trait Trait {}

impl Trait for for<'a> fn(&'a u32) {}

fn f()
where
    for<'a> fn(&'a u32): Trait,
{
}

fn main() {
    f();
}
