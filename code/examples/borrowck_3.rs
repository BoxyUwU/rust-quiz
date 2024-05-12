fn foo(a: &mut u32) -> u32 {
    let mut b = || &mut *a;

    *b() = 12;
    *b() = 73;
    *a
}

fn main() {
    foo(&mut 292);
}
