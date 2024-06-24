struct A;
struct B();
struct C {}

fn main() {
    A {};
    A { .. } = A { ..A };

    B {};
    C {};

    A;
    B;
    C;
}
