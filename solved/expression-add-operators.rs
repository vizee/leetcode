impl Solution {
    fn resolve(res: &mut Vec<String>, ops: &mut Vec<(u8, i64)>, num: &[i64], target: i64, opd: i64) {
        static OPS: [&'static str; 4] = ["", "+", "-", "*"];
        
        if num.is_empty() {
            if target == 0 {
                res.push(ops.iter().map(|(o, a)| format!("{}{}", OPS[*o as usize], a)).collect());
            }
            return;
        }
        let mut x = 0i64;
        for i in 0..num.len() {
            if i == 1 && x == 0 {
                break;
            }
            x = x * 10 + num[i];
            for op in 1..=3 {
                ops.push((op, x));
                match op {
                    1 => Solution::resolve(res, ops, &num[i + 1..], target - x, x),
                    2 => Solution::resolve(res, ops, &num[i + 1..], target + x, -x),
                    3 => Solution::resolve(res, ops, &num[i + 1..], target + opd - opd * x, opd * x),
                    _ => {}
                }
                ops.pop();
            }
        }
    }

    pub fn add_operators(num: String, target: i32) -> Vec<String> {
        if num.is_empty() {
            return Vec::new();
        }
        let num = num.into_bytes().into_iter().map(|c| (c - b'0') as i64).collect::<Vec<_>>();
        let mut res = Vec::new();
        let mut x = 0i64;
        for i in 0..num.len() {
            if i == 1 && x == 0 {
                break;
            }
            x = x * 10 + num[i];
            Self::resolve(&mut res, &mut vec![(0, x)], &num[i + 1..], target as i64 - x, x);
        }
        res
    }
}
