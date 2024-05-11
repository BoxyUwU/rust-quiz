#![allow(deref_nullptr)]

fn main() {
    // 1.  UB?
    unsafe {
        _ = *std::ptr::null::<u32>();
    }
}
