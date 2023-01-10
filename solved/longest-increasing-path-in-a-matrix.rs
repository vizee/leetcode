impl Solution {
    fn search(matrix: &Vec<Vec<i32>>, vis: &mut Vec<Vec<i32>>, i: usize, j: usize) -> i32 {
        const DIRS: [(isize, isize); 4] = [(-1, 0), (0, -1), (1, 0), (0, 1)];
        if vis[i][j] != 0 {
            return vis[i][j];
        }
        vis[i][j] = 1;
        for (u, v) in DIRS.iter() {
            let x = i as isize + u;
            let y = j as isize + v;
            if x < 0 || x >= matrix.len() as isize || y < 0 || y >= matrix[0].len() as isize {
                continue;
            }
            let x = x as usize;
            let y = y as usize;
            if matrix[x][y] > matrix[i][j] {
                vis[i][j] = vis[i][j].max(Self::search(matrix, vis, x, y) + 1);
            }
        }
        vis[i][j]
    }

    pub fn longest_increasing_path(matrix: Vec<Vec<i32>>) -> i32 {
        let n = matrix.len();
        let m = matrix[0].len();
        let mut vis = vec![vec![0i32; m]; n];
        let mut res = 0;
        for i in 0..n {
            for j in 0..m {
                res = res.max(Self::search(&matrix, &mut vis, i, j));
            }
        }
        res
    }
}
