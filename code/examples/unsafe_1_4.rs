#![allow(deref_nullptr)]

fn main() {
    // 4.  UB?
    _ = unsafe { *std::ptr::null::<u32>() };
}
