impl Solution {
    pub fn count_digit_one(n: i32) -> i32 {
        if n <= 0 {
            return 0;
        }
        static TABLE: [i32; 9] = [1, 20, 300, 4000, 50000, 600000, 7000000, 80000000, 900000000];
        static POW10: [i32; 10] = [1, 10, 100, 1000, 10000, 100000, 1000000, 10000000, 100000000, 1000000000];
        let mut t = n;
        let mut d = [0; 10];
        let mut p = 0usize;
        while t > 0 {
            d[p] = t % 10;
            p += 1;
            t /= 10;
        }
        let d = &d[..p];
        let mut total = 0;
        loop {
            p -= 1;
            if p == 0 {
                total += (d[p] > 0) as i32;
                break;
            }
            if d[p] > 1 {
                total += POW10[p];
            } else if d[p] == 1 {
                total += n % POW10[p] + 1;
            }
            total += d[p] * TABLE[p - 1];
        }
        total
    }
}
