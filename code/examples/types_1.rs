trait Cat {}

// meow!
impl<T> Cat for T {}

fn main() {
    let r = &0;
    require_box(Box::new(r));

    let local = 0;
    let r = &local;
    require_box(Box::new(r));
}

// Cats love boxes.
fn require_box(_a: Box<dyn Cat>) {}
