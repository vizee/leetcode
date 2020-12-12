impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        let mut i = 0usize;
        let mut hi = *heights.first().unwrap_or(&0);
        let mut htop = 0i32; // height[stack.top()]
        let mut res = 0i32;
        let mut stack = Vec::new();
        while i <= heights.len() {
            if htop <= hi {
                htop = hi;
                stack.push(i);
                i += 1;
                hi = *heights.get(i).unwrap_or(&0);
            } else {
                let top = stack.pop().unwrap();
                res = res.max(heights[top] * stack.last().map(|j| {
                    htop = heights[*j];
                    i - *j - 1
                }).unwrap_or_else(|| {
                    htop = 0;
                    i
                }) as i32);
            }
        }
        res
    }
}
