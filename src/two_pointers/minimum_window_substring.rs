use std::collections::HashMap;

pub fn min_window(s: String, t: String) -> String {
    // create a freq map for s
    // when right meet other chars and num_0 == 0, move left to right+1
    // when right meet a char in s, decrement the s freq map
    // when freq(right_char) >= 0, keep moving right char and dont move left
    // when num_0 = s.len(), record the window size
    // and then move left until next char in s
    // when num_0 = 0, move left to right + 1
    // when freq(right_char) < 0, move left until freq(right_char) == 0

    // mistake 1: compare num_0 with t, did not handle the case where t has dupe chars

    let mut t_freq = HashMap::<char, i32>::new();
    for c in t.chars() {
        *t_freq.entry(c).or_insert(0) += 1;
    }

    // todo: improve
    let (mut left, mut num_0, mut res, mut right) = (0, 0, "a".repeat(s.len() + 1), 0);
    while right < s.len() {
        let right_char = s.as_bytes()[right] as char;
        if t_freq.contains_key(&right_char) {
            t_freq.entry(right_char).and_modify(|f| *f -= 1);

            if *t_freq.get(&right_char).unwrap() == 0 {
                num_0 += 1;
            }
            // else if t_freq.get(&right_char).unwrap() < 0 {
            // could not think of a case that it matters
            // }

            if num_0 == t_freq.len() {
                if right - left + 1 < res.len() {
                    // todo: improve
                    res = s[left..=right].to_string();
                }

                let mut left_char = s.as_bytes()[left] as char;
                t_freq.entry(left_char).and_modify(|f| *f += 1);
                if *t_freq.get(&left_char).unwrap() > 0 {
                    num_0 -= 1;
                }
                // todo: improve
                left += 1;
                if left >= s.len() {
                    break;
                }
                left_char = s.as_bytes()[left] as char;

                while left < s.len() && !t_freq.contains_key(&left_char) {
                    left += 1;
                    left_char = s.as_bytes()[left] as char;
                }

                // re-evaluate the window after shrinking
                // todo: improve
                t_freq.entry(right_char).and_modify(|f| *f += 1);
                if *t_freq.get(&right_char).unwrap() > 0 {
                    num_0 -= 1;
                }
                // right -= 1;
                continue;
            }
        } else {
            if num_0 == 0 {
                left = right + 1;
            }
        }
        right += 1;
    }
    if res == "a".repeat(s.len() + 1) {
        String::from("")
    } else {
        res
    }

}

#[cfg(test)]
mod test {
    use crate::two_pointers::minimum_window_substring::min_window;

    #[test]
    fn test() {
        assert_eq!(
            "BANC",
            min_window(String::from("AB"), String::from("A"))
        );
    }
}
