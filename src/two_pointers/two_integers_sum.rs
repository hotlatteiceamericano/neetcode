pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    // two pointers?
    let (mut i, mut j) = (0 as usize, numbers.len() - 1);
    while i < j {
        let sum = numbers.get(i).copied().unwrap() + numbers.get(j).copied().unwrap();
        if sum == target {
            return vec![(i + 1) as i32, (j + 1) as i32];
        } else if sum < target {
            i += 1;
        } else {
            j -= 1;
        }
    }
    panic!("no valid solution");
}
