impl Solution {
    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        const FULL_MASK: u32 = (1 << 9) - 1;
        struct Context<'a> {
            cols: [u32; 9],
            rows: [u32; 9],
            fields: [[u32; 3]; 3],
            pending: Vec<(usize, usize)>,
            board: &'a mut Vec<Vec<char>>,
        };
        let mut ctx = Context {
            cols: [0u32; 9],
            rows: [0u32; 9],
            fields: [[0u32; 3]; 3],
            pending: Vec::new(),
            board,
        };
        for i in 0..9 {
            for j in 0..9 {
                let c = ctx.board[i][j];
                if c != '.' {
                    let b = 1 << (c as u32 - '1' as u32);
                    ctx.rows[i] |= b;
                    ctx.cols[j] |= b;
                    ctx.fields[i / 3][j / 3] |= b;
                } else {
                    ctx.pending.push((i, j));
                }
            }
        }
        fn search(u: usize, c: &mut Context) -> bool {
            if u == c.pending.len() {
                for i in 0..9 {
                    if c.cols[i] != FULL_MASK || c.rows[i] != FULL_MASK || c.fields[i / 3][i % 3] != FULL_MASK {
                        return false;
                    }
                }
                return true;
            }
            let (i, j) = c.pending[u];
            let marked = c.rows[i] | c.cols[j] | c.fields[i / 3][j / 3];
            for k in 0..9 {
                if marked & (1 << k) != 0 {
                    continue;
                }
                c.board[i][j] = ('1' as u32 + k) as u8 as char;
                let t = 1 << k;
                c.rows[i] |= t;
                c.cols[j] |= t;
                c.fields[i / 3][j / 3] |= t;

                if search(u + 1, c) {
                    return true;
                }

                let t = !(1 << k);
                c.rows[i] &= t;
                c.cols[j] &= t;
                c.fields[i / 3][j / 3] &= t;
            }
            false
        }
        search(0, &mut ctx);
    }
}
