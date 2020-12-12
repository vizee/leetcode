impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort();
        let mut max = 0;
        let mut n = 1;
        for i in 1..=nums.len() {
            if i < nums.len() && nums[i] == nums[i - 1] {
                continue;
            }
            if i == nums.len() || nums[i] != nums[i - 1] + 1 {
                max = max.max(n);
                n = 1;
            } else {
                n += 1;
            }
        }
        max
    }
}
