#![allow(deref_nullptr)]

fn main() {
    // 5. UB?
    _ = unsafe { *std::ptr::null::<u32>() };
}
