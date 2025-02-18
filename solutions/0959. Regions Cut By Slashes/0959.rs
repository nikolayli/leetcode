// Time complexity: O(n^2)
// Space complexity: O(n^2)
impl Solution {
    pub fn regions_by_slashes(grid: Vec<String>) -> i32 {
        let n = grid.len();
        let mut g = vec![vec![0; n * 3]; n * 3];

        for i in 0..n {
            for j in 0..n {
                match grid[i].chars().nth(j).unwrap() {
                    '/' => {
                        g[i * 3][j * 3 + 2] = 1;
                        g[i * 3 + 1][j * 3 + 1] = 1;
                        g[i * 3 + 2][j * 3] = 1;
                    }
                    '\\' => {
                        g[i * 3][j * 3] = 1;
                        g[i * 3 + 1][j * 3 + 1] = 1;
                        g[i * 3 + 2][j * 3 + 2] = 1;
                    }
                    _ => (),
                }
            }
        }

        fn dfs(g: &mut Vec<Vec<i32>>, i: usize, j: usize) {
            if i < 0 || i >= g.len() || j < 0 || j >= g[0].len() {
                return;
            }
            if g[i][j] != 0 {
                return;
            }
            g[i][j] = 2;
            dfs(g, i + 1, j);
            dfs(g, i - 1, j);
            dfs(g, i, j + 1);
            dfs(g, i, j - 1);
        }

        let mut ans = 0;
        for i in 0..(n * 3) {
            for j in 0..(n * 3) {
                if g[i][j] == 0 {
                    dfs(&mut g, i, j);
                    ans += 1;
                }
            }
        }

        ans
    }
}
