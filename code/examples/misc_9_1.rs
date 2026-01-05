#![allow(dead_code)]

// ANCHOR:matters
struct Struct {}
struct Tuple(u32);
struct Unit;

fn main() {
    Struct;
    Tuple;
    Unit;
}
// ANCHOR_END:matters
