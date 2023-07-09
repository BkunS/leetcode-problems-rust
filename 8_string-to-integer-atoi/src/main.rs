struct Solution {}

impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let chars = s.chars();
        let mut negative = false;
        let mut leading_num = false;
        let mut result: String = String::new();

        for n in chars {
            match n {
                '0'..='9' => {
                    leading_num = true;
                    result.push(n);
                }
                '-' => {
                    if leading_num {
                        break;
                    } else {
                        leading_num = true;
                        negative = true;
                    }
                }
                '+' => {
                    if leading_num {
                        break;
                    }
                    leading_num = true;
                }
                ' ' => {
                    if leading_num {
                        break;
                    }
                }
                _ => break,
            };
        }
        if negative {
            result = String::from('-') + &result
        }
        match result.parse::<i32>() {
            Ok(n) => n,
            Err(msg) => match msg.kind() {
                std::num::IntErrorKind::PosOverflow => i32::MAX,
                std::num::IntErrorKind::NegOverflow => i32::MIN,
                _ => 0,
            },
        }
    }
}

fn main() {
    let s = String::from("-+12");
    let result = Solution::my_atoi(s);
    println!("{}", result);
}
