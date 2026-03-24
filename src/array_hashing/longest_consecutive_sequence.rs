use std::collections::HashMap;

pub fn solution(nums: Vec<i32>) -> i32 {
    let mut length_so_far = HashMap::new();
    let mut longest = 0;
    for num in nums {
        if length_so_far.contains_key(&num) {
            continue;
        }
        let left_bound_len = length_so_far.get(&(num - 1)).copied().unwrap_or(0);
        let right_bound_len = length_so_far.get(&(num + 1)).copied().unwrap_or(0);
        let combined_len = 1 + left_bound_len + right_bound_len;

        length_so_far.insert(num, combined_len);

        longest = longest.max(combined_len);

        if length_so_far.contains_key(&(num - left_bound_len)) {
            length_so_far.insert(num - left_bound_len, combined_len);
        }
        if length_so_far.contains_key(&(num + right_bound_len)) {
            length_so_far.insert(num + right_bound_len, combined_len);
        }
    }

    longest
}

pub fn solution_that_is_wrong_but_dont_have_mutable_and_immutable_borrow_at_the_same_time(
    nums: Vec<u32>,
) -> i32 {
    let mut length_so_far = HashMap::new();
    let mut longest = 0;
    for num in nums {
        if length_so_far.contains_key(&num) {
            continue;
        }
        let left_bound_len = length_so_far.get(&(num - 1)).unwrap_or(&0);
        let right_bound_len = length_so_far.get(&(num + 1)).unwrap_or(&0);
        let combined_len = 1 + *left_bound_len + *right_bound_len;

        length_so_far.insert(num, combined_len);

        longest = longest.max(combined_len);

        if length_so_far.contains_key(&(num - 1)) {
            length_so_far.insert(num - 1, combined_len);
        }
        if length_so_far.contains_key(&(num + 1)) {
            length_so_far.insert(num + 1, combined_len);
        }
    }

    longest
}
