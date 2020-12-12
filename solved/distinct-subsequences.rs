impl Solution {
    pub fn num_distinct(s: String, t: String) -> i32 {
        if s.len() < t.len() {
            return 0;
        }
        let s = s.into_bytes();
        let t = t.into_bytes();
        let n = s.len();
        let m = t.len();
        let mut dp = Vec::new();
        dp.resize(n * m, 0);
        let mut first = 0;
        for i in 0..n {
            first += (t[0] == s[i]) as i32;
            dp[i * m] = first;
        }
        if first == 0 {
            return 0;
        }
        for i in 1..n {
            for j in 1..m {
                if j > i {
                    break;
                }
                dp[i * m + j] = if s[i] == t[j] {
                    dp[(i - 1) * m + j - 1] + dp[(i - 1) * m + j]
                } else {
                    dp[(i - 1) * m + j]
                }
            }
        }
        *dp.last().unwrap_or(&0)
    }
}
