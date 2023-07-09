struct Solution {}

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x.is_negative() {
            return false;
        }

        let str = x.to_string();
        let mut start: usize = 0;
        let mut end: usize = str.len() - 1;

        while start < end {
            if str.chars().nth(start) != str.chars().nth(end) {
                return false;
            }
            start += 1;
            end -= 1;
        }

        true
    }
}

fn main() {
    let x = 121;
    let result = Solution::is_palindrome(x);
    println!("Result: {}", result);
}
