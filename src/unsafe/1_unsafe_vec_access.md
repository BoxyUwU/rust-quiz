# Unsafe 1

The idea for this question was suggested by [`@orlp`](https://github.com/orlp/).

```rust
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

    // SAFETY: The vec has 10 elements, so 1 and 2 are in bounds.
    //         The two elements are distinct, so we are not overlapping.
    unsafe {
        let r1: *mut u8 = v.get_unchecked_mut(1);
        let r2: *mut u8 = v.get_unchecked_mut(2);
        *r1 = 10;
        *r2 = 10;
    }
}
```

<details>
<summary>Solution</summary>

```
error: Undefined Behavior: attempting a write access using <2611> at alloc1167[0x1], but that tag does not exist in the borrow stack for this location
  --> src/main.rs:18:9
   |
18 |         *r1 = 10;
   |         ^^^^^^^^
   |         |
   |         attempting a write access using <2611> at alloc1167[0x1], but that tag does not exist in the borrow stack for this location
   |         this error occurs as part of an access at alloc1167[0x1..0x2]
   |
   = help: this indicates a potential bug in the program: it performed an invalid operation, but the Stacked Borrows rules it violated are still experimental
   = help: see https://github.com/rust-lang/unsafe-code-guidelines/blob/master/wip/stacked-borrows.md for further information
help: <2611> was created by a SharedReadWrite retag at offsets [0x1..0x2]
  --> src/main.rs:16:27
   |
16 |         let r1: *mut u8 = v.get_unchecked_mut(1);
   |                           ^^^^^^^^^^^^^^^^^^^^^^
help: <2611> was later invalidated at offsets [0x0..0xa] by a Unique retag
  --> src/main.rs:17:27
   |
17 |         let r2: *mut u8 = v.get_unchecked_mut(2);
   |                           ^^^^^^^^^^^^^^^^^^^^^^
   = note: BACKTRACE (of the first span):
   = note: inside `main` at src/main.rs:18:9: 18:17

note: some details are omitted, run with `MIRIFLAGS=-Zmiri-backtrace=full` for a verbose backtrace

error: aborting due to 1 previous error
```

`v.as_mut_pointer()` refers to `Vec::as_mut_ptr(v)`, which only creates a `&mut` reference to the vec itself.

`v.get_unchecked_mut()` comes from an implicit dereference to a slice, referring to `<[T]>::get_unchecked_mut(Vec::deref(v))`, creating a `&mut` to all slice elements.
When doing the deref for creating `r2`, this full `&mut` invalidates `r1`.

</details>
