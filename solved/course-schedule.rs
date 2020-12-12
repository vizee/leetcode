impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        use std::collections::vec_deque::VecDeque;
        let mut adj = Vec::new();
        adj.resize(num_courses as usize, Vec::new());
        let mut indegree = Vec::new();
        indegree.resize(num_courses as usize, 0);
        for pair in prerequisites {
            adj[pair[0] as usize].push(pair[1] as usize);
            indegree[pair[1] as usize] += 1;
        }
        let mut q = VecDeque::with_capacity(num_courses as usize / 2);
        for i in 0..indegree.len() {
            if indegree[i] == 0 {
                q.push_back(i);
            }
        }
        let mut finished = 0;
        while let Some(u) = q.pop_front() {
            finished += 1;
            for &v in &adj[u] {
                indegree[v] -= 1;
                if indegree[v] == 0 {
                    q.push_back(v);
                }
            }
        }
        finished == num_courses
    }
}
