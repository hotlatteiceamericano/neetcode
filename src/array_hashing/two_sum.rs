use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    // Map<remaining, index>
    let mut map: HashMap<i32, i32> = HashMap::new();
    for i in 0..nums.len() {
        let num = nums[i];
        let remaining = target - num;
        if let Some(j) = map.get(&remaining) {
            return vec![*j, i as i32];
        }
        map.insert(num, i as i32);
    }
    panic!("no matching pairs")
}
