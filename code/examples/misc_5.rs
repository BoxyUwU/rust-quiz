trait Super {
    fn assoc() -> Self;
}

trait Sub: Super {}

fn f<T: Sub>() -> T {
    Sub::assoc()
}

fn main() {}
