struct A;
struct B();
struct C {}

fn main() {
    A {};
    B {};
    C {};

    A;
    B;
    C;
}
