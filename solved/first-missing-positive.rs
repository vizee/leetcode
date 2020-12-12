impl Solution {
    pub fn first_missing_positive(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort();
        let mut n = 1;
        for x in nums {
            if x < n {
                continue;
            }
            if n == x {
                n = x + 1;
            } else {
                break;
            }
        }
        return n;
    }
}
