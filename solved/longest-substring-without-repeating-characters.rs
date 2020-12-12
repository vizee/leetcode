impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        if s.is_empty() {
            return 0;
        }
        let s = s.as_bytes();
        let mut r = 0;
        let mut p = 0;
        let mut b = 0;
        let mut v = [-1; 128];
        for i in 0..s.len() {
            let t = v[s[i] as usize];
            v[s[i] as usize] = p;
            if t >= b {
                if p - b > r {
                    r = p - b;
                }
                b = t + 1;
            }
            p += 1;
        }
        if p - b > r { p - b } else { r }
    }
}
