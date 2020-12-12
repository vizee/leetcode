impl Solution {
    pub fn add_digits(num: i32) -> i32 {
        if (num < 10) {
            return num;
        }

        let mut num = num;
        while num >= 10 {
            let mut n = 0;
            while num > 0 {
                n += num % 10;
                num /= 10;
            }
            num = n;
        }
        return num;
    }
}
