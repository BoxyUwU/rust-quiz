#![allow(deref_nullptr)]

// ANCHOR:matters
fn main() {
    // 5. UB?
    _ = unsafe { *std::ptr::null::<u32>() };
}
// ANCHOR_END:matters
