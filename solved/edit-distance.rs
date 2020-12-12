// https://leetcode.com/problems/edit-distance/
impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let n = word1.len() + 1;
        let m = word2.len() + 1;
        let mut d = Vec::new();
        d.resize(n * m, 0);
        for i in 0..n {
            d[i * m] = i as i32;
        }
        for j in 0..m {
            d[j] = j as i32;
        }
        let word1 = word1.as_bytes();
        let word2 = word2.as_bytes();
        for i in 1..n {
            for j in 1..m {
                d[i * m + j] = (d[(i - 1) * m + j - 1] + (word1[i - 1] != word2[j - 1]) as i32)
                    .min(d[i * m + j - 1] + 1)
                    .min(d[(i - 1) * m + j] + 1);
            }
        }
        d[n * m - 1]
    }
}
