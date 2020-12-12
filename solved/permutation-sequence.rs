impl Solution {
    pub fn get_permutation(n: i32, k: i32) -> String {
        static F: [i32; 9] = [
            1,
            1 * 2,
            1 * 2 * 3,
            1 * 2 * 3 * 4,
            1 * 2 * 3 * 4 * 5,
            1 * 2 * 3 * 4 * 5 * 6,
            1 * 2 * 3 * 4 * 5 * 6 * 7,
            1 * 2 * 3 * 4 * 5 * 6 * 7 * 8,
            1 * 2 * 3 * 4 * 5 * 6 * 7 * 8 * 9,
        ];
        let mut t = ['1', '2', '3', '4', '5', '6', '7', '8', '9'];
        let mut pick = move |mut i| {
            for j in 0..t.len() {
                if t[j] == '0' {
                    continue;
                }
                if i == 0 {
                    let c = t[j];
                    t[j] = '0';
                    return c;
                }
                i -= 1;
            }
            unreachable!()
        };
        let mut res = String::new();
        let mut k = k - 1;
        let mut d = (n - 1) as usize;
        while d > 0 {
            d -= 1;
            let x = k / F[d];
            res.push(pick(x as usize));
            k = k % F[d];
        }
        res.push(pick(0));
        res
    }
}
