use core::pin::Pin;
use core::marker::PhantomPinned;

#[derive(Default)]
struct MyPinnedType {
    addr: usize,
    _pin: PhantomPinned,
}

impl MyPinnedType {
    fn my_pin_fn(self: Pin<&mut Self>) {
        let me = unsafe { Pin::into_inner_unchecked(self) };
        let me_addr = me as *mut Self as usize;
        if me.addr == 0 {
            me.addr = me_addr;
        } else {
            assert_eq!(me.addr, me_addr, "Pinned value was moved.")
        }
    }
}

trait MyUnpinTrait {
    fn into_pinned_type(self: Pin<&mut Self>) -> Pin<&mut MyPinnedType>;
}
impl MyUnpinTrait for MyPinnedType {
    fn into_pinned_type(self: Pin<&mut Self>) -> Pin<&mut MyPinnedType> {
        self
    }
}
impl Unpin for dyn MyUnpinTrait {}

fn main() {
    let mut pinned_type = MyPinnedType::default();
    
    Pin::new((&mut pinned_type) as &mut dyn MyUnpinTrait).into_pinned_type().my_pin_fn();
    
    let pinned_type = Box::new(pinned_type);
    Pin::from(pinned_type).as_mut().my_pin_fn();
}