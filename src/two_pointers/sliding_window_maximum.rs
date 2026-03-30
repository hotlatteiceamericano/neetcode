use std::collections::BinaryHeap;

pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
    // think to use a max heap of size k
    // each time the window shift, it pop the number from heap and insert a new num
    // after popping out and inserting, get the biggest by peeking the top of the heap
    // each operation is (logk), and iterate through nums takes O(n) => O(nlogk)

    // but I need heap to maintain the order, while also poping out a specific number

    // solution: remove all the biggest numbers, until a num's index is within the window
    // index + k <= right means index is out of window
    // with max heap, I now the max number of current window
    // however I don't know the index of the max number
    // what if I store the index together in the heap?
    // then I know the index of the current max number
    // I can then understand if that max number is still in the window
    // by calculating the current window range
    // curr window range is [i-k+1..i]
    // [1,3,6,7,4,6,2,1,8,5,8]

    // so the "hard" part of this question is
    // how do you know the biggest number is stili within the window
    // I can know which number is the biggest number
    // I can then know if it is still in the window, by also storing the index in the heap
    // then, I can calculate that index is still in the window or not, by:
    // heap.peek().1 >= i-k+1
    // if the biggest number is not within the window, pop it out
    // can keep doing this until the next biggest number is in the window

    let mut heap = BinaryHeap::new();
    let mut res = Vec::new();

    for i in 0..nums.len() {
        heap.push((nums[i], i));
        if (i as i32) - k + 1 >= 0 {
            // index larger than window size
            while (i as i32) - k + 1 > heap.peek().unwrap().1 as i32 {
                // max number's index is smaller than window left edge, meaning out of the window
                heap.pop();
            }
            res.push(heap.peek().unwrap().0);
        }
    }

    res
}
