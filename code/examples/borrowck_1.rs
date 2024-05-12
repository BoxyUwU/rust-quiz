fn func(a: &'static u32) {
    dbg!(a);
}

fn accepts_func(f: fn(&u32), data: &u32) {
    f(data);
}

fn main() {
    //  &'static u32 can be shortened to any
    // lifetime borrow so func should be fine
    accepts_func(func, &274);
}
