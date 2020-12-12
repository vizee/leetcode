impl Solution {
    pub fn number_of_lines(widths: Vec<i32>, s: String) -> Vec<i32> {
        s.as_bytes().iter().map(|&c| widths[(c - b'a') as usize]).fold(vec![1, 0], |mut acc, w| {
            acc[1] += w;
            if acc[1] > 100 {
                acc[0] += 1;
                acc[1] = w;
            }
            acc
        })
    }
}
