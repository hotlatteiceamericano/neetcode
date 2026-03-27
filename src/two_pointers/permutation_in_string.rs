use std::collections::HashMap;

pub fn check_inclusion(s1: String, s2: String) -> bool {
    // save s1's char's frequencies in a map
    // iterate s2, when right pointer sees a char exist in the map
    // decrement that char's frequency
    // when char's frequency < 0, meanning the substring is not a permutation of s1
    // move the left pointer until that char's freq >= 0
    // whenever seeing a char's freq is 0, calculate how many 0 so far
    // if number equals to s1.len(), return true

    // when iterating s2, it can only be:
    // 1) seeing a char not in the s1
    // 2) seeing a char more times than in the s1

    // I was using num_0 to keep track of which char has appear enough times
    // which is fine
    // howeverm using num_0 to compare with s1.len() is wrong
    // as number of distinct chars != s1.len()
    // when s1 contains dupes

    let mut s1_freq = HashMap::<char, i32>::new();
    for c in s1.chars() {
        *s1_freq.entry(c).or_insert(0) += 1;
    }
    let mut s1_freq_clone = s1_freq.clone();

    let mut left = 0usize;

    for right in 0..s2.len() {
        let right_char = s2.chars().collect::<Vec<char>>()[right];
        if s1_freq_clone.contains_key(&right_char) {
            s1_freq_clone.entry(right_char).and_modify(|f| *f -= 1);

            while *s1_freq_clone.get(&right_char).unwrap() < 0 {
                let left_char = s2.chars().collect::<Vec<char>>()[left];
                s1_freq_clone.entry(left_char).and_modify(|f| *f += 1);
                left += 1;
            }

            if right - left + 1usize == s1.len() {
                return true;
            }
        } else {
            while left < s2.len() && left <= right {
                let left_char = s2.chars().collect::<Vec<char>>()[left];
                s1_freq_clone.entry(left_char).and_modify(|f| *f += 1);
                left += 1;
            }
        }
    }

    false
}
