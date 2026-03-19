use std::collections::HashSet;

pub fn has_duplicate(nums: Vec<i32>) -> bool {
    let mut set = HashSet::<i32>::new();
    for num in nums {
        if !set.insert(num) {
            return true;
        }
    }
    false
}
