use std::collections::HashMap;

pub fn character_replacement(s: String, k: i32) -> i32 {
    // first thought
    // so in another word, allowing repeating chars to have k more chars adjacent
    // then when j moves, it decrement k by 1 when seeing a
    // when i moves, it has to keep moving until k becomes > 0

    // second thought
    // [aaabbbb], k=3
    // so if the solution is to keep the most frequent chars, then I will have to keep track of most frequent chars
    // then the number to replace becomes window length - most frequent length
    // then compare k with number of replacements
    // if num_replacement > k, move i to num_replacement <= k

    // reflect
    // I need to keep track of longest char length
    // I don't necessarily need to keep tracck of longest char, to know the longest char length

    let (mut i, mut j) = (0i32, 0i32);
    let mut longest_len = 0;
    let mut freq = HashMap::new();
    let mut res = 0i32;

    while (j as usize) < s.len() {
        let char_j = s.chars().collect::<Vec<char>>()[j as usize];

        *freq.entry(char_j).or_insert(0) += 1;

        if *freq.get(&char_j).unwrap() > longest_len {
            longest_len = *freq.get(&char_j).unwrap();
        }

        let mut num_replacement = (j - i + 1) - longest_len;
        while num_replacement > k {
            let char_i = s.chars().collect::<Vec<char>>()[i as usize];
            freq.entry(char_i).and_modify(|f| *f -= 1);
            i += 1;

            num_replacement = (j - i + 1) - longest_len;
        }

        res = res.max(j - i + 1);

        j += 1;
    }

    res
}
