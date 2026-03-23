fn solution(nums: Vec<i32>) -> Vec<i32> {
    let mut had_zero = false;
    let product = nums
        .iter()
        .copied()
        .reduce(|acc, n| {
            if n == 0 {
                if had_zero {
                    return 0;
                } else {
                    had_zero = true;
                    return acc;
                }
            }
            return acc * n;
        })
        .expect("empty nums");

    if product == 0 {
        return vec![0; nums.len()];
    }

    nums.into_iter()
        .map(|n| {
            if product == 0 {
                return 0;
            }
            if n == 0 {
                return n;
            }
            return product / n;
        })
        .collect()
}
