impl Solution {
    pub fn resolve(g: &Vec<Vec<i32>>, output: &mut Vec<Vec<i32>>, path: &mut Vec<i32>, cur: usize) {
        if cur == g.len() - 1 {
            output.push(path.clone());
        } else {
            g[cur].iter().for_each(|&next| {
                path.push(next);
                Self::resolve(g, output, path, next as usize);
                path.pop();
            });
        }
    }

    pub fn all_paths_source_target(graph: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut res = Vec::new();
        Self::resolve(&graph, &mut res, &mut vec![0], 0);
        res
    }
}
