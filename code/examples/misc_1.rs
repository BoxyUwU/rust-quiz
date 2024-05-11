#![allow(unreachable_code)]

struct PrintOnDrop(&'static str);
impl Drop for PrintOnDrop {
    fn drop(&mut self) {
        eprint!("{}", self.0);
    }
}

fn main() {
    owo();
    uwu();
}

fn owo() {
    (
        (PrintOnDrop("1"), PrintOnDrop("2"), PrintOnDrop("3")),
        return,
    );
}

fn uwu() {
    (PrintOnDrop("1"), PrintOnDrop("2"), PrintOnDrop("3"), return);
}
