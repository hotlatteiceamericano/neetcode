use std::collections::HashMap;

fn soluttion(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut map = HashMap::<i32, i32>::new();
    for num in &nums {
        map.entry(*num).and_modify(|o| *o += 1).or_insert(1);
    }

    let mut group_by_occur = vec![Vec::<i32>::new(); nums.len() + 1];
    for (key, value) in map.iter() {
        group_by_occur[*value as usize].push(*key);
    }

    let mut ans = Vec::<i32>::new();
    for (_i, v) in group_by_occur.iter().rev().enumerate() {
        if !v.is_empty() {
            ans.extend(v);
        }
        if ans.len() >= k as usize {
            return ans[..k as usize].to_vec();
        }
    }
    ans[..k as usize].to_vec()
}
