impl Solution {
    pub fn max_points(points: Vec<Vec<i32>>) -> i32 {
        fn slope_of(p0: &Vec<i32>, p1: &Vec<i32>) -> (i32, i32) {
            let x = p1[0] - p0[0];
            let y = p1[1] - p0[1];
            if x == 0 {
                return (0, 1);
            }
            if y == 0 {
                return (1, 0);
            }
            let mut a = x;
            let mut b = y;
            while b != 0 {
                let t = b;
                b = a % b;
                a = t;
            }
            (x / a, y / a)
        }

        let mut max = 0;
        let mut slopes = HashMap::with_capacity(points.len());
        for i in 0..points.len() {
            slopes.clear();
            let mut overlap = 1;
            let p0 = &points[i];
            for j in i + 1..points.len() {
                if *p0 == points[j] {
                    overlap += 1;
                    continue;
                }
                let slope = slope_of(p0, &points[j]);
                match slopes.get_mut(&slope) {
                    Some(v) => {
                        *v += 1;
                    }
                    None => {
                        slopes.insert(slope, 1);
                    }
                }
            }
            max = max.max(*slopes.values().into_iter().max().unwrap_or(&0) + overlap);
        }
        max
    }
}
