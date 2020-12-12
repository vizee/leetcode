impl Solution {
    fn largest_rectangle_area(heights: &Vec<i32>) -> i32 {
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

    pub fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32 {
        let mut heights = Vec::new();
        heights.resize(matrix.first().map_or(0, |x| x.len()), 0);
        let mut res = 0i32;
        for row in matrix {
            for i in 0..row.len() {
                heights[i] = if row[i] == '1' { heights[i] + 1 } else { 0 };
            }
            let a = Self::largest_rectangle_area(&heights);
            if a > res {
                res = a;
            }
        }
        res
    }
}
