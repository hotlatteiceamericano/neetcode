use std::collections::HashMap;

// forget about about how to de-dupe during the first attempt of using HashMap
// it is important to observe that, to return NUMBERS, or to return INDICES!
// returning numbers indicates that it can manipulate the array order
fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut nums = nums.clone();
    nums.sort();

    let mut res = Vec::new();
    for (i, target) in nums.iter().enumerate() {
        if i > 0 && nums[i] == nums[i - 1] {
            continue;
        }
        let (mut j, mut k) = (i + 1, nums.len() - 1);
        let remains = 0 - target;
        while j < k {
            let sum = nums[j] + nums[k];
            if sum < remains {
                j += 1;
            } else if sum > remains {
                k -= 1;
            } else {
                res.push(vec![*target, nums[j], nums[k]]);
                j += 1;
                k -= 1;
                while nums[j - 1] == nums[j] && nums[k + 1] == nums[k] && j < k {
                    j += 1;
                    k -= 1;
                }
            }
        }
    }
    res
}

fn map_solution(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut map = HashMap::new();
    for (i, num) in nums.iter().enumerate() {
        map.entry(num).or_insert(Vec::new()).push(i); // make sure the mutable borrow ends here
    }

    let mut nums = nums.clone();
    nums.sort();

    let mut res = Vec::new();
    for (i, target) in nums.iter().enumerate() {
        for (j, num_j) in nums.iter().enumerate() {
            if i == j {
                continue;
            }

            let remains = 0 - *target - *num_j;
            let remain_nums: Vec<i32> = map
                .get(&remains)
                .unwrap_or(&Vec::new())
                .iter()
                .copied()
                .filter(|k| *k != i && *k != j)
                .map(|k| nums.get(k).copied().unwrap())
                .collect();
            if remain_nums.is_empty() {
                continue;
            }

            let ans: Vec<Vec<i32>> = remain_nums
                .iter()
                .copied()
                .map(|n| vec![*target, *num_j, n])
                .collect();
            res.extend(ans);
        }
    }
    res
}
