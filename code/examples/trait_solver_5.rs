trait Super {}
trait Sub<T>: Super {}

trait Overlap<T> {}
impl<T, U: Sub<T>> Overlap<T> for U {}
impl<T> Overlap<T> for () {}

fn main() {}
