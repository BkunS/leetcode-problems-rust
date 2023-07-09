use std::cmp;

struct Solution {}

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut max = 0;
        let mut start = 0;
        let mut end = height.len() - 1;

        while start < end {
            let curr_min = cmp::min(height[start], height[end]);
            max = cmp::max(max, curr_min * (end - start) as i32);
            if height[start] < height[end] {
                start += 1;
            } else {
                end -= 1;
            }
        }
        max
    }
}

fn main() {
    let height = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];
    let result = Solution::max_area(height);
    println!("Result: {}", result);
}
