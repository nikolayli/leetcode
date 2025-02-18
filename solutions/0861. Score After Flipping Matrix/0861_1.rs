// Time Complexity: O(mn)
// Space Complexity: O(1)
impl Solution {
    pub fn matrix_score(mut grid: Vec<Vec<i32>>) -> i32 {
        let rows = grid.len();
        let cols = grid[0].len();

        for row in grid.iter_mut() {
            if row[0] == 0 {
                Self::flip(row);
            }
        }

        for j in 0..cols {
            let mut sum = 0;
            for i in 0..rows {
                sum += grid[i][j];
            }
            if sum * 2 < rows as i32 {
                Self::flip_col(&mut grid, j);
            }
        }

        grid.into_iter().map(Self::binary).sum()
    }

    fn flip(row: &mut Vec<i32>) {
        for val in row.iter_mut() {
            *val ^= 1;
        }
    }

    fn flip_col(grid: &mut Vec<Vec<i32>>, j: usize) {
        for row in grid.iter_mut() {
            row[j] ^= 1;
        }
    }

    fn binary(row: Vec<i32>) -> i32 {
        row.into_iter().fold(0, |acc, b| acc * 2 + b)
    }
}
