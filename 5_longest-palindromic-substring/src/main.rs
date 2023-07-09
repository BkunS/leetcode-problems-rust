use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let char_vec: Vec<char> = s.chars().collect();
        let len = char_vec.len();
        if len < 2 {
            return s;
        }
        for (index, curr) in char_vec.into_iter().enumerate() {
            // TODO:
        }
        String::from("")
    }
}

fn main() {
    let str = String::from("cbbd");
    let result = Solution::longest_palindrome(str);
    println!("Output: {}", result);
}
