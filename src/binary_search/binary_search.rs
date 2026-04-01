pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    let (mut i, mut j) = (0usize, nums.len() - 1);
    while i < j {
        let mid = (i + j) / 2;
        if nums[mid] == target {
            return mid as i32;
        } else if nums[mid] < target {
            i = mid + 1;
        } else {
            j = mid;
        }
    }

    if nums[i] == target {
        return i as i32;
    } else {
        return -1;
    }
}
