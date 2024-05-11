fn main() {
    let mut v = vec![0_u8; 10];

    // SAFETY: The vec has 10 elements, so 1 and 2 are in bounds.
    //         The two elements are distinct, so we are not overlapping.
    unsafe {
        let r1: *mut u8 = v.as_mut_ptr().add(1);
        let r2: *mut u8 = v.as_mut_ptr().add(2);
        *r1 = 10;
        *r2 = 10;
    }
}
