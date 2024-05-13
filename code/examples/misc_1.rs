pub unsafe const fn my_zeroed<T>() -> T {
    assert!(std::mem::size_of::<T>() <= 256);
    std::mem::transmute_copy(&[0u8; 256])
}

fn main() {
    unsafe {
        my_zeroed::<[*mut u32; 64]>();
    }
}
