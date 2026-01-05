#![allow(unreachable_code)]

// ANCHOR:matters
struct Struct {}
struct Tuple();
struct Unit;

fn main() {
    Struct = loop {};
    Tuple = loop {};
    Unit = loop {};
}
// ANCHOR_END:matters
