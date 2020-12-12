impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        static TABLE: [&'static str; 10] = [" ", "", "abc", "def", "ghi", "jkl", "mno", "pqrs", "tuv", "wxyz"];
        digits.into_bytes()
            .iter()
            .map(|c| TABLE[*c as usize - '0' as usize])
            .fold(Vec::new(), |v, x| if v.is_empty() {
                x.chars().map(|c| c.to_string()).collect()
            } else {
                v.iter().flat_map(|s| x.chars().map(|c| {
                    let mut s = s.clone();
                    s.push(c);
                    s
                }).collect::<Vec<String>>()).collect()
            })
    }
}
