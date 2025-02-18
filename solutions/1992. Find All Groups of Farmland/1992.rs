// Time complexity: O(mn)
// Space complexity: O(mn)
impl Solution {
    pub fn find_farmland(mut land: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut ans: Vec<Vec<i32>> = Vec::new();

        fn dfs(i: usize, j: usize, cell: &mut Vec<i32>, land: &mut Vec<Vec<i32>>) {
            let rows = land.len();
            let cols = land[0].len();

            if i >= rows || j >= cols || land[i][j] != 1 {
                return;
            }

            land[i][j] = 2;
            cell[0] = cell[0].max(i as i32);
            cell[1] = cell[1].max(j as i32);

            dfs(i + 1, j, cell, land);
            dfs(i, j + 1, cell, land);
        }

        let rows = land.len();
        let cols = land[0].len();

        for i in 0..rows {
            for j in 0..cols {
                if land[i][j] == 1 {
                    let mut cell: Vec<i32> = vec![i as i32, j as i32];
                    dfs(i, j, &mut cell, &mut land);
                    ans.push(vec![i as i32, j as i32, cell[0], cell[1]]);
                }
            }
        }

        ans
    }
}
