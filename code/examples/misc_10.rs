#[repr(u32)]
enum A {
    Zero,
    One(),
    Two {},
}

fn main() {
    println!("{}", A::Zero as usize);
    println!("{}", A::One as usize);
    println!("{}", A::Two as usize);

    println!("{}", A::Zero {} as usize);
    println!("{}", A::One {} as usize);
    println!("{}", A::Two {} as usize);
}
