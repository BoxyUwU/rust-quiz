pub trait Super {}
pub trait Sub<T>: Super {}

pub trait Overlap<T> {}
impl<T, U: Sub<T>> Overlap<T> for U {}
impl<T> Overlap<T> for () {}

fn main() {}
