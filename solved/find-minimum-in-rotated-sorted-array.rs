impl Solution {
    fn find_min_s(nums: &[i32]) -> i32 {
        match nums.len() {
            0 => 0,
            1 => nums[0],
            n => {
                if nums[0] < nums[n - 1] {
                    nums[0]
                } else {
                    let k = n / 2;
                    Self::find_min_s(&nums[0..k]).min(Self::find_min_s(&nums[k..]))
                }
            }
        }
    }

    pub fn find_min(nums: Vec<i32>) -> i32 {
        Self::find_min_s(&nums)
    }
}
