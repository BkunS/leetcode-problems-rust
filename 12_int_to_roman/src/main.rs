struct Solution {}

impl Solution {
    pub fn int_to_roman(num: i32) -> String {
        let mut curr = num;
        let mut ret = String::new();
        while curr > 0 {
            match curr {
                1000.. => {
                    ret.push('M');
                    curr -= 1000;
                }
                900..=999 => {
                    ret += "CM";
                    curr -= 900;
                }
                500..=899 => {
                    ret.push('D');
                    curr -= 500;
                }
                400..=499 => {
                    ret += "CD";
                    curr -= 400;
                }
                100..=399 => {
                    ret.push('C');
                    curr -= 100;
                }
                90..=99 => {
                    ret += "XC";
                    curr -= 90;
                }
                50..=89 => {
                    ret.push('L');
                    curr -= 50;
                }
                40..=49 => {
                    ret += "XL";
                    curr -= 40;
                }
                10..=39 => {
                    ret.push('X');
                    curr -= 10;
                }
                9 => {
                    ret += "IX";
                    curr -= 9;
                }
                5..=8 => {
                    ret.push('V');
                    curr -= 5;
                }
                4 => {
                    ret += "IV";
                    curr -= 4;
                }
                1..=3 => {
                    ret.push('I');
                    curr -= 1;
                }
                _ => (),
            }
        }
        ret
    }
}

fn main() {
    let num = 1994;
    let result = Solution::int_to_roman(num);
    println!("Result: {}", result);
}
