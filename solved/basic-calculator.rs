// https://leetcode.com/problems/basic-calculator/
impl Solution {
    fn calc_expr(s: &[u8]) -> (i32, usize) {
        let mut nparen = 0;
        let mut op = '+';
        let mut sum = 0;
        let mut operand = 0;
        let mut i = 0usize;
        while i < s.len() {
            let c = s[i] as char;
            i += 1;
            if c == ' ' {
                continue;
            }
            if c.is_ascii_digit() {
                operand = operand * 10 + (c as u8 - '0' as u8) as i32;
                continue;
            }
            if c == '(' {
                if op == '-' {
                    let (subsum, n) = Self::calc_expr(&s[i..]);
                    sum -= subsum;
                    i += n;
                } else {
                    nparen += 1;
                }
                continue;
            }
            if op == '+' {
                sum += operand;
            }else {
                sum -= operand;
            }
            operand = 0;
            if c == ')' {
                if nparen > 0 {
                    nparen -= 1;
                } else {
                    break;
                }
            } else {
                op = c;
            }
        }
        (if op == '+' {
            sum + operand
        } else {
            sum - operand
        }, i)
    }

    pub fn calculate(s: String) -> i32 {
        let (sum, _) = Self::calc_expr(s.as_bytes());
        sum
    }
}
