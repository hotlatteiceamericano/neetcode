pub fn trap(height: Vec<i32>) -> i32 {
    // amount of water can be trapped is decided by
    // left and right bars
    // and also the shorter of the left and right bar
    // the amount of water a particular point can store
    // is the highest bar from my left - my length
    // + highest bar from my right - my length
    // * width diff - 1
    // but how do each bar know if it can add up with previoud water trapped?
    // reset the trapped water whenever I cannot trap any water?
    // meaning if both left and right bar are not higher than me, reset
    let mut from_left = vec![0; height.len()];
    let mut from_right = vec![0; height.len()];

    let mut max = 0;
    for (i, h) in height.iter().enumerate() {
        from_left[i] = max;
        max = max.max(*h);
    }
    max = 0;
    for (i, h) in height.iter().rev().enumerate() {
        from_right[i] = max;
        max = max.max(*h);
    }

    let mut water_trapped = 0;
    for (i, h) in height.iter().enumerate() {
        let curr = from_left[i].min(from_right[i]) - h;

        if curr > 0 {
            water_trapped += curr;
        }
    }

    water_trapped
}
