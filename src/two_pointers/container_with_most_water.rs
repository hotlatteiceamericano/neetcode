pub fn max_area(heights: Vec<i32>) -> i32 {
    // amount of water = min(l_height, r_height) * width
    // since it is decided by smaller height
    // moving the smaller height has higher chance to find larger amount of water

    // another proof of this solution is
    // the largest amount will be at the two most furthrest points and it happend them to be the highest
    let (mut i, mut j) = (0, heights.len() - 1);
    let mut res = 0;
    while i < j {
        let amount = heights[i].min(heights[j]) * (j as i32 - i as i32);
        res = res.max(amount);
        if heights[i] < heights[j] {
            i += 1;
        } else {
            j -= 1;
        }
    }
    res
}
