impl Solution {
    pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
        let mut nums = nums;
        nums.sort();
        let mut res = nums[0..3].iter().sum::<i32>();
        let mut d = (res - target).abs();
        for i in 0..nums.len() - 2 {
            let mut j = i + 1;
            let mut k = nums.len() - 1;
            while j < k {
                let r2 = nums[i] + nums[j] + nums[k];
                let d2 = r2 - target;
                if d2 == 0 {
                    return target;
                }
                if d2.abs() < d {
                    res = r2;
                    d = d2.abs();
                }
                if d2 > 0 {
                    k -= 1;
                } else {
                    j += 1;
                }
            }
        }
        res
    }
}
