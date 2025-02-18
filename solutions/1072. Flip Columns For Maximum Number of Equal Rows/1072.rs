// Time complexity: O(mn)
// Space complexity: O(mn)
use std::collections::HashMap;

impl Solution {
    pub fn max_equal_rows_after_flips(matrix: Vec<Vec<i32>>) -> i32 {
        let mut pattern_frequency = HashMap::new();

        for row in &matrix {
            let mut pattern = String::new();
            for &col in row {
                if col == row[0] {
                    pattern.push('T');
                } else {
                    pattern.push('F');
                }
            }
            *pattern_frequency.entry(pattern).or_insert(0) += 1;
        }

        *pattern_frequency.values().max().unwrap_or(&0)
    }
}
