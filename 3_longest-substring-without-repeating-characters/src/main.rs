use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut hm = HashMap::new();
        let mut max_len: i32 = 0;
        let mut start = -1;

        for (end, ch) in s.chars().enumerate() {
            if let Some(index) = hm.insert(ch, end as i32) {
                start = start.max(index);
            }
            max_len = max_len.max(end as i32 - start);
        }
        max_len
    }
}

fn main() {
    let s = String::from("pwwkew");
    let result = Solution::length_of_longest_substring(s);
    println!("Result: {}", result);
}
