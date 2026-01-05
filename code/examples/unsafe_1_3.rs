#![allow(deref_nullptr, unused_parens)]

// ANCHOR:matters
fn main() {
    // 3. UB?
    unsafe {
        _ = (*std::ptr::null::<u32>(),);
    }
}
// ANCHOR_END:matters
