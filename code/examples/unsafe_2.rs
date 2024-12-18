use std::mem::MaybeUninit;

fn main() {
    // Safety: all bitpatterns are valid for u8
    let random_number: u8 = unsafe { MaybeUninit::uninit().assume_init() };

    let very_random_number = if random_number <= 100 {
        unsafe {
            // Safety: all bitpatterns are valid for u32
            let rng_array: [u32; 100] = MaybeUninit::uninit().assume_init();
            // Safety: The `random_number` is in bounds
            *rng_array.get_unchecked(random_number as usize)
        }
    } else {
        // chosen by a fair dice roll
        4
    };

    dbg!(very_random_number);
}
