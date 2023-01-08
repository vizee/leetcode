impl Solution {
    fn count_and_sort(sums: &mut [i64], buf: &mut [i64], lower: i64, upper: i64) -> usize {
        if sums.len() <= 1 {
            return 0;
        }
        let mid = sums.len() / 2;
        let n1 = Self::count_and_sort(&mut sums[..mid], buf, lower, upper);
        let n2 = Self::count_and_sort(&mut sums[mid..], buf, lower, upper);
        let mut l = mid;
        let mut r = mid;
        let mut n = n1 + n2;
        for &v in sums[..mid].iter() {
            while l < sums.len() && sums[l] - v < lower {
                l += 1;
            }
            while r < sums.len() && sums[r] - v <= upper {
                r += 1;
            }
            n += r - l;
        }
        let mut i = 0;
        let mut j = mid;
        let mut k = 0usize;
        while k < sums.len() {
            if i >= mid || (j < sums.len() && sums[j] < sums[i]) {
                buf[k] = sums[j];
                j += 1;
            } else {
                buf[k] = sums[i];
                i += 1;
            }
            k += 1;
        }
        sums.copy_from_slice(&buf[..sums.len()]);

        n
    }

    pub fn count_range_sum(nums: Vec<i32>, lower: i32, upper: i32) -> i32 {
        let mut sum = 0i64;
        let mut sums = Vec::with_capacity(nums.len() + 1);
        sums.push(0);
        for x in nums {
            sum += x as i64;
            sums.push(sum);
        }
        let mut buf = vec![0i64; sums.len()];
        Self::count_and_sort(&mut sums, &mut buf, lower as i64, upper as i64) as i32
    }
}
