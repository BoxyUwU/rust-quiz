trait TraitA {}
trait TraitB<T: Sized> {}

impl TraitA for &dyn TraitB<[u8]> {}

trait MyDefault {
    fn default() -> Self;
}

impl<T: TraitA> MyDefault for T {
    fn default() -> T {
        todo!("not important for the example")
    }
}

fn main() {
    <&dyn TraitB<[u8]> as MyDefault>::default();
}
