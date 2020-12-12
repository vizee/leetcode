impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = Vec::with_capacity(s.len() / 2);
        s.into_bytes().iter().map(|&c| if c == b'(' { b'\'' } else { c }).all(|c| match c {
            b'\'' | b'[' | b'{' => stack.push(c + 2) == (),
            _ => stack.pop().map_or(false, |x| x == c),
        }) && stack.is_empty()
    }
}
