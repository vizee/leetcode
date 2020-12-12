impl Solution {
    pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
        if s1.len() + s2.len() != s3.len() {
            return false;
        }
        use std::collections::vec_deque::VecDeque;
        let s1 = s1.into_bytes();
        let s2 = s2.into_bytes();
        let s3 = s3.into_bytes();
        let mut q = VecDeque::with_capacity(s3.len());
        let mut visit = Vec::new();
        let n = s1.len() + 1;
        let m = s2.len() + 1;
        visit.resize(n * m, false);
        q.push_back((0, 0));
        while let Some((i, j)) = q.pop_front() {
            if i + j == s3.len() {
                return true;
            }
            if i < s1.len() && s1[i] == s3[i + j] && !visit[(i + 1) * m + j] {
                q.push_back((i + 1, j));
                visit[(i + 1) * m + j] = true;
            }
            if j < s2.len() && s2[j] == s3[i + j] && !visit[i * m + j + 1] {
                q.push_back((i, j + 1));
                visit[i * m + j + 1] = true;
            }
        }
        false
    }
}
