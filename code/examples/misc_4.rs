fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
    assert_eq!(map.get_mut(&1), Some(&mut 2));
}

