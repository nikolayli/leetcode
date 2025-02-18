// Time complexity: O(mn)
// Space complexity: O(mn)
use std::cmp::Ordering;

impl Solution {
    pub fn min_falling_path_sum(mut grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();

        for i in 1..n {
            let two_min_num_and_indices = Self::get_two_min_num_and_indices(&grid[i - 1]);
            let (first_min_num, first_min_index) = two_min_num_and_indices[0];
            let (second_min_num, _) = two_min_num_and_indices[1];

            for j in 0..n {
                if j == first_min_index {
                    grid[i][j] += second_min_num;
                } else {
                    grid[i][j] += first_min_num;
                }
            }
        }

        *grid.last().unwrap().iter().min().unwrap()
    }

    fn get_two_min_num_and_indices(nums: &[i32]) -> Vec<(i32, usize)> {
        let mut num_and_indices: Vec<(i32, usize)> =
            nums.iter().enumerate().map(|(i, &num)| (num, i)).collect();
        num_and_indices.sort_unstable_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(Ordering::Equal));

        vec![num_and_indices[0], num_and_indices[1]]
    }
}
