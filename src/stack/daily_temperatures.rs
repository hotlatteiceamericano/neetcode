pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
    // basically the position of next bigger number
    // brute force of O(n^2)
    // when iterating a new number, it can only be larger, or smaller (or equal)
    // when seeing a larger number, update before until the day, who also is a larger number, and update the current largest
    // when seeing a equal or smaller number, mark itself that it is not a bigger number, and reset the curr_largest to itself
    // or similar to the min stack, record the current biggest at each number
    // so when seeing a larger number, it traverse back to update its distance until their curr max is bigger
    // [100, 90, 40, 30, 20, 50, 110]
    let mut vec: Vec<(i32, usize)> = Vec::new();
    let mut res = vec![0; temperatures.len()];
    for (i, t) in temperatures.iter().enumerate() {
        while !vec.is_empty() && *t > vec.last().unwrap().0 {
            let colder_day = vec.pop().unwrap();
            res[colder_day.1] = (i - colder_day.1) as i32;
        }

        vec.push((*t, i));
    }
    res
}
