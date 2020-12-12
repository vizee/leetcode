impl Solution {
    pub fn min_cut(s: String) -> i32 {
        let s = s.as_bytes();
        let mut dp = Vec::new();
        dp.resize(s.len() + 1, s.len() as i32);
        dp[0] = -1;
        for i in 0..s.len() {
            let mut j = 0;
            while i >= j && i + j < s.len() && s[i - j] == s[i + j] {
                dp[i + j + 1] = dp[i + j + 1].min(dp[i - j] + 1).min(dp[i + j] + 1);
                j += 1;
            }
            let mut j = 0;
            while i >= j + 1 && i + j < s.len() && s[i - j - 1] == s[i + j] {
                dp[i + j + 1] = dp[i + j + 1].min(dp[i - j - 1] + 1).min(dp[i + j] + 1);
                j += 1;
            }
        }
        dp[dp.len() - 1]
    }
}
