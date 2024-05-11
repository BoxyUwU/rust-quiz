struct A;
struct B;
struct C;

trait Trait<'a, T> {
    type Assoc;
}

trait Bound {}

impl Bound for B {}
impl Bound for C {}

impl<'a> Trait<'a, B> for A {
    type Assoc = B;
}

impl<'a> Trait<'a, C> for A {
    type Assoc = C;
}

fn f<T>(_: T)
where
    for<'a> A: Trait<'a, T>,
    for<'a> <A as Trait<'a, T>>::Assoc: Bound,
{
}

fn main() {
    f(B);
}
