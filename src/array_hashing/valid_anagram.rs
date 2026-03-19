use std::collections::HashMap;

pub fn is_anagram(s: String, t: String) -> bool {
    let mut map = build_occur_map(s);
    for c in t.chars() {
        match map.get_mut(&c) {
            None => return false,
            Some(v) => {
                *v -= 1;
                if *v == 0 {
                    map.remove(&c);
                }
            }
        }
    }
    map.is_empty()
}

pub fn build_occur_map(s: String) -> HashMap<char, i32> {
    let mut map = HashMap::<char, i32>::new();
    for c in s.chars() {
        map.entry(c).and_modify(|v| *v += 1).or_insert(1);
    }
    map
}
