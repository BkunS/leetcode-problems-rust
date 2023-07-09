use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut hash_map: HashMap<i32, usize> = HashMap::with_capacity(nums.len());
        for (i, &num) in nums.iter().enumerate() {
            match hash_map.get(&num) {
                Some(&target_index) => return vec![i as i32, target_index as i32],
                None => {
                    hash_map.insert(target - num, i);
                }
            }
        }
        unreachable!();
    }
}

fn main() {
    let nums = vec![2, 7, 11, 15];
    let target = 9;
    let result = Solution::two_sum(nums, target);
    println!("Output: {:?}", result);
}
