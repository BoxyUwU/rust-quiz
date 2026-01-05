fn func(a: &'static u32) {
    dbg!(a);
}

fn accepts_func(f: for<'a> fn(&'a u32), data: &u32) {
    f(data);
}

fn main() {
    let local = 274;
    //  &'static u32 can be shortened to any
    // lifetime borrow so func should be fine
    accepts_func(func, &local);
}
