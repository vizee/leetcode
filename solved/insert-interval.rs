impl Solution {
    pub fn insert(intervals: Vec<Interval>, new_interval: Interval) -> Vec<Interval> {
        if intervals.is_empty() {
            return vec![new_interval];
        }
        if new_interval.end < intervals[0].start {
            let mut intervals = intervals;
            intervals.insert(0, new_interval);
            return intervals;
        }
        let mut used = false;
        let mut ni = new_interval;
        let mut r = Vec::with_capacity(intervals.len());
        for i in intervals {
            let i = if i.end < ni.start {
                i
            } else if ni.end < i.start {
                if !used {
                    r.push(Interval::new(ni.start, ni.end));
                    used = true;
                }
                i
            } else {
                if i.start < ni.start {
                    ni.start = i.start;
                }
                if i.end > ni.end {
                    ni.end = i.end;
                }
                continue;
            };
            r.push(i);
        }
        if !used {
            r.push(ni);
        }
        r
    }
}
