impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        enum Part<'a> {
            Char(u8),
            Str(&'a str),
            Star,
            Skip(usize),
        }
        const STAR: u8 = '*' as u8;
        const QUEST: u8 = '?' as u8;
        let ps = p.as_str();
        let pb = p.as_bytes();
        let mut parts = Vec::<Part>::with_capacity(pb.len());
        let mut i = 0usize;
        while i < pb.len() {
            match pb[i] {
                STAR => {
                    while i + 1 < pb.len() && pb[i + 1] == STAR {
                        i += 1;
                    }
                    parts.push(Part::Star);
                }
                QUEST => {
                    let mut n = 1usize;
                    while i + 1 < pb.len() && pb[i + 1] == QUEST {
                        i += 1;
                        n += 1;
                    }
                    parts.push(Part::Skip(n));
                }
                _ => {
                    let mut b = i;
                    while i + 1 < pb.len() && pb[i + 1] != STAR && pb[i + 1] != QUEST {
                        i += 1;
                    }
                    if b == i {
                        parts.push(Part::Char(pb[i]));
                    } else {
                        parts.push(Part::Str(&ps[b..=i]));
                    }
                }
            }
            i += 1;
        }
        let sb = s.as_bytes();
        let mut stack = vec![(0usize, 0usize)];
        let mut star = false;
        while let Some((mut i, mut j)) = stack.pop() {
            while i < sb.len() && j < parts.len() {
                match parts[j] {
                    Part::Char(b) => if star {
                        star = false;
                        if let Some(pos) = s[i..].find(b as char) {
                            i += pos + 1;
                            stack.push((i, j));
                        } else {
                            return false;
                        }
                    } else if sb[i] == b {
                        i += 1
                    } else {
                        break;
                    }
                    Part::Str(x) => if star {
                        star = false;
                        if let Some(pos) = s[i..].find(x) {
                            i += pos;
                            stack.push((i + 1, j));
                            i += x.len();
                        } else {
                            return false;
                        }
                    } else if s[i..].starts_with(x) {
                        i += x.len();
                    } else {
                        break;
                    }
                    Part::Skip(n) => i += n,
                    Part::Star => star = true,
                }
                j += 1;
            }
            while j < parts.len() {
                if let Part::Star = parts[j] {
                    j += 1;
                    star = true;
                } else {
                    star = false;
                    break;
                }
            }
            if (i == sb.len() && j == parts.len()) || (i < sb.len() && j == parts.len() && star) {
                return true;
            }
            star = true;
        }
        false
    }
}
