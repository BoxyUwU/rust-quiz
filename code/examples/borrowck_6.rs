type R1<'a> = &'a mut i32;
struct R2<'a>(&'a mut i32);

fn r1_once(_: R1){}
fn r2_once(_: R2){}

pub fn r1_twice(x: R1) {
    r1_once(x);
    r1_once(x);
}

pub fn r2_twice(x: R2) {
    r2_once(x);
    r2_once(x);
}

fn main() {
    r1_twice(&mut 0);
    r2_twice(R2(&mut 0));
}
