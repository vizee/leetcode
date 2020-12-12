impl Solution {
    fn get_area(h: &Vec<i32>, start: usize, end: usize) -> i32 {
        let x = h[start].min(h[end]);
        let mut sum = 0i32;
        for i in start..=end {
            sum += x - x.min(h[i]);
        }
        sum
    }

    fn subtrap(h: &Vec<i32>, mut start: usize, mut end: usize) -> i32 {
        if end - start < 2 {
            return 0;
        }
        let p;
        if h[start] >= h[end] {
            p = start;
            start += 1;
        } else {
            p = end;
            end -= 1;
        };
        let mut max = start;
        for i in start..=end {
            if h[i] > h[max] {
                max = i;
            }
        }
        if p < max {
            Self::get_area(h, p, max) + Self::subtrap(h, max, end)
        } else {
            Self::get_area(h, max, p) + Self::subtrap(h, start, max)
        }
    }

    pub fn trap(height: Vec<i32>) -> i32 {
        if height.len() < 3 {
            return 0;
        }
        let mut max = 0usize;
        for i in 0..height.len() {
            if height[i] > height[max] {
                max = i;
            }
        }
        return Self::subtrap(&height, 0, max) + Self::subtrap(&height, max, height.len() - 1);
    }
}
