fn main() {
    let map = HashMap::new();
    map.insert(1, 2);
    assert_eq!(map.get_mut(1), Some(&2));
}
