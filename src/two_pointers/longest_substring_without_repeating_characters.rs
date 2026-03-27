use std::collections::HashMap;

pub fn length_of_longest_substring(s: String) -> i32 {
    // sliding window?
    // when seeing a unique char, keep moving the front pointer
    // when seeing a dupe char, record length, compare with longest and move the rear pointer
    // the point being, move the read pointer to where?
    // I think it should not be moved to where the front pointer is at,
    // rather, move to where the dupe char is at + 1
    // with that, use a map to store seen chars and their index?

    if s.len() == 0 {
        return 0;
    }

    let (mut i, mut j) = (0, 0);
    let mut map = HashMap::new();
    let mut longest = 0;

    while j < s.len() {
        let char_j = s.chars().collect::<Vec<char>>()[j];

        if let Some(seen_char_index) = map.insert(char_j, j) {
            // or i = i.max(seen_char_index) + 1
            // point being i cannot move backward
            if seen_char_index >= i {
                i = seen_char_index + 1;
            }
        }

        longest = longest.max(j - i + 1);

        j += 1;
    }

    longest as i32
}
