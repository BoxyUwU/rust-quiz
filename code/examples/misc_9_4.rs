struct Struct {}
struct Tuple();
struct Unit;

type TypeStruct = Struct;
type TypeTuple = Tuple;
type TypeUnit = Unit;

fn main() {
    TypeStruct {};
    TypeTuple();
    TypeUnit;
}
