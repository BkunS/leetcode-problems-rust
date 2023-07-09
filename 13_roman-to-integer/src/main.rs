struct Solution {}

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut ret = 0;
    }
}

fn main() {
    let s = String::from("MCMXCIV");
    let result = Solution::roman_to_int(s);
    println!("Result: {}", result);

    assert!(result == 1994);
}
