macro_rules! capture {
    ($e:expr) => {
        transform!($e)
    };
}

macro_rules! transform {
    ($a:tt + $b:tt) => {
        $a * $b
    };
    ($other:expr) => {
        $other
    };
}

fn main() {
    // what will be output of this macro calls
    dbg!(capture!(2 + 3));
    dbg!(transform!(2 + 3));
}
