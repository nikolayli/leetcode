// Time complexity: O(1)
// Space complexity: O(1)
impl Solution {
    pub fn can_make_square(grid: Vec<Vec<char>>) -> bool {
        for i in 0..grid.len() - 1 {
            for j in 0..grid[0].len() - 1 {
                let (mut b, mut w) = (0, 0);
                for (u, v) in [(i, j), (i + 1, j), (i, j + 1), (i + 1, j + 1)] {
                    b += (grid[u][v] == 'B') as i32;
                    w += (grid[u][v] == 'W') as i32;
                }
                if b >= 3 || w >= 3 {
                    return true;
                }
            }
        }

        false
    }
}
