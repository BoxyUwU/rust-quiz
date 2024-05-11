#![allow(deref_nullptr, unused_braces)]

fn main() {
    // 3.  UB?
    unsafe {
        _ = { *std::ptr::null::<u32>() };
    }
}
