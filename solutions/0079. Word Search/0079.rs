// Time complexity: O(mn4^k)
// Space complexity: O(4^k)
impl Solution {
    pub fn exist(mut board: Vec<Vec<char>>, word: String) -> bool {
        fn dfs(board: &mut Vec<Vec<char>>, i: usize, j: usize, word: &str) -> bool {
            if word.is_empty() {
                return true;
            }

            let cache = board[i][j];
            board[i][j] = '#';
            if i > 0 && word.chars().next().unwrap() == board[i - 1][j] {
                if dfs(board, i - 1, j, &word[1..]) {
                    return true;
                }
            }
            if i < board.len() - 1 && word.chars().next().unwrap() == board[i + 1][j] {
                if dfs(board, i + 1, j, &word[1..]) {
                    return true;
                }
            }
            if j > 0 && word.chars().next().unwrap() == board[i][j - 1] {
                if dfs(board, i, j - 1, &word[1..]) {
                    return true;
                }
            }
            if j < board[0].len() - 1 && word.chars().next().unwrap() == board[i][j + 1] {
                if dfs(board, i, j + 1, &word[1..]) {
                    return true;
                }
            }

            board[i][j] = cache;

            false
        }

        for i in 0..board.len() {
            for j in 0..board[0].len() {
                if board[i][j] == word.chars().next().unwrap() && dfs(&mut board, i, j, &word[1..])
                {
                    return true;
                }
            }
        }

        false
    }
}
