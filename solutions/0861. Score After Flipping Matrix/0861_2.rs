// Time Complexity: O(mn)
// Space Complexity: O(1)
impl Solution {
    pub fn matrix_score(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut ans = m as i32;

        for j in 1..n {
            let ones_count = grid
                .iter()
                .map(|row| (row[j] == row[0]) as i32)
                .sum::<i32>();
            ans = ans * 2 + std::cmp::max(ones_count, m as i32 - ones_count);
        }

        ans
    }
}
