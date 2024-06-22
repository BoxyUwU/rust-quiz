#![allow(deref_nullptr, unused_parens)]

fn main() {
    // 3. UB?
    unsafe {
        _ = (*std::ptr::null::<u32>(),);
    }
}
