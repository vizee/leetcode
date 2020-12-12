impl Solution {
    pub fn network_delay_time(times: Vec<Vec<i32>>, n: i32, k: i32) -> i32 {
        const INF: i32 = 20000;

        let n = n as usize;
        let mut g = Vec::with_capacity(n + 1);
        for i in 0..g.capacity() {
            let mut row = Vec::new();
            row.resize(n + 1, -1);
            g.push(row);
        }
        let mut d = Vec::new();
        d.resize(n + 1, INF);
        let mut visit = Vec::new();
        visit.resize(n + 1, false);
        for x in &times {
            g[x[0] as usize][x[1] as usize] = x[2];
        }
        let end = n + 1;
        let mut cur = k as usize;
        d[cur] = 0;
        loop {
            visit[cur] = true;
            for i in 1..=n {
                if g[cur][i] == -1 {
                    continue;
                }
                if d[cur] + g[cur][i] < d[i] {
                    d[i] = d[cur] + g[cur][i];
                }
            }
            let mut next = INF;
            for i in 1..=n {
                if visit[i] {
                    continue;
                }
                if d[i] < next {
                    next = d[i];
                    cur = i;
                }
            }
            if next == INF {
                break;
            }
        }
        let mut maxd = 0;
        for i in 1..=n {
            if d[i] == INF {
                return -1;
            }
            maxd = maxd.max(d[i]);
        }
        maxd
    }
}
