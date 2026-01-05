#![allow(deref_nullptr, unused_braces)]

// ANCHOR:matters
fn main() {
    // 4. UB?
    unsafe {
        _ = { *std::ptr::null::<u32>() };
    }
}
// ANCHOR_END:matters
