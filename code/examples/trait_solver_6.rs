fn func(a: &u32) {
    dbg!(a);
}

fn accepts_func(b: fn(&u32), c: &u32) {
    b(c);
}

fn main() {
    accepts_func(func as fn(_), &23);
}
