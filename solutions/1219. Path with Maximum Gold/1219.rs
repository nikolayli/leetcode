// Time complexity: O(mn*maxPath)
// Space complexity: O(mn)
impl Solution {
    pub fn get_maximum_gold(mut grid: Vec<Vec<i32>>) -> i32 {
        fn dfs(grid: &mut Vec<Vec<i32>>, i: usize, j: usize) -> i32 {
            if i < 0 || j < 0 || i >= grid.len() || j >= grid[0].len() || grid[i][j] == 0 {
                return 0;
            }

            let gold = grid[i][j];
            grid[i][j] = 0;

            let paths = [
                dfs(grid, i + 1, j),
                dfs(grid, i - 1, j),
                dfs(grid, i, j + 1),
                dfs(grid, i, j - 1),
            ];
            let max_path = *paths.iter().max().unwrap();

            grid[i][j] = gold;
            gold + max_path
        }

        let mut ans = 0;
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                ans = ans.max(dfs(&mut grid, i, j));
            }
        }

        ans
    }
}
