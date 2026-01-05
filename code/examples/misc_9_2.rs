#![allow(dead_code)]

// ANCHOR:matters
struct Struct {}
struct Tuple(u32);
struct Unit;

fn main() {
    Struct {};
    Tuple { 0: 1_u32 };
    Unit {};
}
// ANCHOR_END:matters
