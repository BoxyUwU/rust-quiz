use std::collections::HashMap;

fn get_or_insert_1234(map: &mut HashMap<u32, u64>) -> &mut u64 {
    if let Some(v) = map.get_mut(&0) {
        return v;
    }

    map.insert(0, 1234);
    map.get_mut(&0).unwrap()
}

fn main() {}
