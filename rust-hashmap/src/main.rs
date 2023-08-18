use std::collections::HashMap;
use multimap::MultiMap;

fn main() {
    let mut _hm: HashMap<String, String> = HashMap::new();
    // let mut _mm: MultiMap<String, Vec<String>> = MultiMap::new();
    let mut _mm: MultiMap<String, String> = MultiMap::new();

    _hm.insert(String::from("1"), String::from("test1"));
    _hm.insert(String::from("1"), String::from("test2"));

    let key = String::from("2");
    _mm.insert(key.clone(), String::from("test1"));
    _mm.insert(key.clone(), String::from("test2"));
    let _values = _mm.get_vec(&key);

    println!("Hello, world!");
}
