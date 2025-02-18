// Time complexity: O(mn)
// Space complexity: O(mn)
impl Solution {
    pub fn count_sub_islands(grid1: Vec<Vec<i32>>, mut grid2: Vec<Vec<i32>>) -> i32 {
        fn dfs(grid1: &Vec<Vec<i32>>, grid2: &mut Vec<Vec<i32>>, i: isize, j: isize) -> i32 {
            if i < 0 || i >= grid2.len() as isize || j < 0 || j >= grid2[0].len() as isize {
                return 1;
            }
            if grid2[i as usize][j as usize] != 1 {
                return 1;
            }

            grid2[i as usize][j as usize] = 2;

            (dfs(grid1, grid2, i + 1, j)
                & dfs(grid1, grid2, i - 1, j)
                & dfs(grid1, grid2, i, j + 1)
                & dfs(grid1, grid2, i, j - 1)
                & grid1[i as usize][j as usize])
        }

        let mut ans = 0;

        for i in 0..grid2.len() {
            for j in 0..grid2[0].len() {
                if grid2[i][j] == 1 {
                    ans += dfs(&grid1, &mut grid2, i as isize, j as isize);
                }
            }
        }

        ans
    }
}
