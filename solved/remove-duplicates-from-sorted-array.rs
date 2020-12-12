impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        let mut prev = nums[0];
        let mut p = 1usize;
        for i in 1..nums.len() {
            if prev != nums[i] {
                prev = nums[i];
                nums[p] = nums[i];
                p += 1;
            }
        }
        nums.truncate(p);
        p as i32
    }
}
