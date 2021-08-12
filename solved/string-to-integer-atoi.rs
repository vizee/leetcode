impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let t = s.as_bytes();
        let mut i = 0;
        while i < t.len() && t[i] == b' ' {
            i += 1;
        }
        let mut sign = 1;
        let mut base = 0i32;
        match t.get(i) {
            Some(b'-') => {
                sign = -1;
                i += 1;
            }
            Some(b'+') => {
                i += 1;
            }
            _ => {}
        }
        let p = i;
        while i < t.len() {
            let c = t[i];
            if !c.is_ascii_digit() {
                break;
            }
            let n = base.wrapping_mul(10).wrapping_add((c - b'0') as i32);
            if base > i32::MAX / 10 || n < base {
                return if sign == 1 { i32::MAX } else { i32::MIN };
            }
            base = n;
            i += 1;
        }
        if i > p {
            sign * base
        } else {
            0
        }
    }
}
