#![allow(deref_nullptr)]

// ANCHOR:matters
fn main() {
    // 1. UB?
    unsafe {
        _ = *std::ptr::null::<u32>();
    }
}
// ANCHOR_END:matters
