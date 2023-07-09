struct Solution {}

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let is_neg = x.is_negative();
        let mut num_vec: Vec<char> = if is_neg {
            (-x).to_string().chars().collect()
        } else {
            x.to_string().chars().collect()
        };
        let mut reversed: Vec<char> = Vec::new();

        while let Some(n) = num_vec.pop() {
            reversed.push(n)
        }
        let reversed_str: String = reversed.into_iter().collect();
        match reversed_str.parse::<i32>() {
            Ok(n) => {
                if is_neg {
                    -n
                } else {
                    n
                }
            }
            Err(..) => 0,
        }
    }
}

fn main() {
    let x: i32 = -123;
    let result = Solution::reverse(x);
    println!("Result: {}", result);
}
