// Time complexity: O(1)
// Space complexity: O(1)
use std::collections::HashSet;

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut seen: HashSet<String> = HashSet::new();

        for i in 0..board.len() {
            for j in 0..board[0].len() {
                if board[i][j] == '.' {
                    continue;
                }
                let c = board[i][j].to_string();
                if !seen.insert(c.clone() + "@row" + &i.to_string())
                    || !seen.insert(c.clone() + "@col" + &j.to_string())
                    || !seen
                        .insert(c.clone() + "@box" + &(i / 3).to_string() + &(j / 3).to_string())
                {
                    return false;
                }
            }
        }

        true
    }
}
