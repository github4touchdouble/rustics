use std::collections::BTreeMap;

pub fn integrate_counter_dictionaries(map1:BTreeMap<char,u64>, map2: BTreeMap<char,u64>) -> BTreeMap<char, u64> {
    let mut merged_map = map1.clone();
    for (char, count) in map2 {
        *merged_map.entry(char).or_insert(0) += count;
    }
    merged_map
}