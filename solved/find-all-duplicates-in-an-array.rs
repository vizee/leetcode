impl Solution {
    pub fn find_duplicates(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        let mut res = Vec::new();
        let n = nums.len() as i32;
        for i in 0..nums.len() {
            let mut j = nums[i] as usize - 1;
            if j >= nums.len() {
                j -= nums.len();
            }
            if nums[j] > n {
                res.push((j + 1) as i32);
            } else {
                nums[j] += n;
            }
        }
        res
    }
}
