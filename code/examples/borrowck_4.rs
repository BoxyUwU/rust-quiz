#[derive(Debug)]
struct Foo<'a>(&'a mut u32);

fn main() {
    let mut data = 10;
    let mut foo = Foo(&mut data);
    mutate(&mut foo);
    dbg!(foo);
}

fn mutate<'a>(f: &'a mut Foo<'a>) {
    *f.0 += 1;
}
