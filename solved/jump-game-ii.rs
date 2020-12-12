impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let mut pos = 0usize;
        let mut n = 0i32;
        let mut max = 0;
        while pos < nums.len() && max < nums.len() - 1 {
            n += 1;
            let mut next = max;
            for i in pos..=max {
                if i + nums[i] as usize > next {
                    next = i + nums[i] as usize;
                }
            }
            pos = max + 1;
            max = next;
        }
        n
    }
}
