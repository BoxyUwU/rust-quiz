struct Unit;
struct Tuple();
struct Struct {}

fn main() {
    Unit {};
    Tuple {};
    Struct {};

    Unit();
    Tuple();
    Struct();

    Unit;
    Tuple;
    Struct;

    Unit { .. } = Unit { ..Unit };
    Tuple { .. } = Tuple { ..Tuple() };
    Struct { .. } = Struct { ..Struct {} };
}
