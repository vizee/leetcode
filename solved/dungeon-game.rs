impl Solution {
    pub fn calculate_minimum_hp(dungeon: Vec<Vec<i32>>) -> i32 {
        let mut hp = dungeon;
        let m = hp[0].len();
        for i in (0..hp.len()).rev() {
            for j in (0..m).rev() {
                let mut cur = hp.get(i + 1).map(|row| row[j]);
                cur = hp[i].get(j + 1).and_then(|&x| cur.map(|v| v.max(x)).or(Some(x))).or(cur);
                hp[i][j] = (cur.unwrap_or(0) + hp[i][j]).min(0);
            }
        }
        1 - hp[0][0]
    }
}
