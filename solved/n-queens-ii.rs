impl Solution {
    // https://leetcode.com/problems/n-queens-ii/
    pub fn total_n_queens(n: i32) -> i32 {
        let n = n as usize;
        let mut cols = Vec::new();
        cols.resize(n, false);
        let mut d = Vec::new();
        d.resize((n + n) as usize, false);
        let mut bd = Vec::new();
        bd.resize((n + n) as usize, false);
        let mut stack = Vec::<usize>::with_capacity(n);
        let mut r = 0i32;
        let mut nexti = 0usize;
        loop {
            let j = stack.len();
            let mut more = false;
            for i in nexti..n {
                if cols[i] || d[n - 1 + i - j] || bd[i + j] {
                    continue;
                }
                if j == n - 1 {
                    r += 1;
                } else {
                    cols[i] = true;
                    d[n - 1 + i - j] = true;
                    bd[i + j] = true;
                    more = true;
                    stack.push(i);
                    break;
                }
            }
            if !more && !stack.is_empty() {
                let i = stack.pop().unwrap();
                let j = stack.len();
                cols[i] = false;
                d[n - 1 + i - j] = false;
                bd[i + j] = false;
                nexti = i + 1;
            } else {
                nexti = 0;
            }
            if stack.is_empty() && nexti == 0 {
                break;
            }
        }
        r
    }
}
