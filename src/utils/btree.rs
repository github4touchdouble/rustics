use std::any::TypeId;
use std::collections::BTreeMap;


// TODO: REMOVE REDUNDANCY AND REPLACE WITH GENERICS
pub fn integrate_f64_counter_dictionaries(map1:BTreeMap<char,f64>, map2: BTreeMap<char,f64>) -> BTreeMap<char, f64> {
    let mut merged_map = map1.clone();
    for (char, count) in map2 {
        *merged_map.entry(char).or_insert(0.0) += count;
    }
    merged_map
}

pub fn integrate_u64_counter_dictionaries(map1:BTreeMap<char,u64>, map2: BTreeMap<char,u64>) -> BTreeMap<char, u64> {
    let mut merged_map = map1.clone();
    for (char, count) in map2 {
        *merged_map.entry(char).or_insert(0) += count;
    }
    merged_map
}