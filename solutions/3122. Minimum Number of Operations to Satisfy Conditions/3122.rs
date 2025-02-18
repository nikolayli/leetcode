// Time complexity: O(rows*cols)
// Space complexity: O(rows*cols)
impl Solution {
    pub fn minimum_operations(grid: Vec<Vec<i32>>) -> i32 {
        let rows = grid.len();
        let cols = grid[0].len();

        let mut count = vec![vec![0; 10]; cols];

        for i in 0..cols {
            for j in 0..rows {
                count[i][grid[j][i] as usize] += 1;
            }
        }

        let mut dp = vec![vec![-1; 11]; 1001];

        let minimum_ops = Self::calculate_minimum_operations(0, 10, rows, cols, &count, &mut dp);

        minimum_ops
    }

    fn calculate_minimum_operations(
        index: usize,
        prev: usize,
        rows: usize,
        cols: usize,
        count: &Vec<Vec<i32>>,
        dp: &mut Vec<Vec<i32>>,
    ) -> i32 {
        if index >= cols {
            return 0;
        }

        if dp[index][prev] != -1 {
            return dp[index][prev];
        }

        let mut ans = std::i32::MAX;

        for i in 0..=9 {
            if i != prev {
                ans = std::cmp::min(
                    ans,
                    rows as i32 - count[index][i]
                        + Self::calculate_minimum_operations(index + 1, i, rows, cols, count, dp),
                );
            }
        }

        dp[index][prev] = ans;

        ans
    }
}
