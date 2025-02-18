// Time complexity: O(n^3)
// Space complexity: O(n^2)
use std::cmp::{max, min};

impl Solution {
    pub fn stone_game_ii(piles: Vec<i32>) -> i32 {
        let n = piles.len();
        let mut mem = vec![vec![0; n]; n];
        let mut suffix = piles.clone();

        for i in (0..n - 1).rev() {
            suffix[i] += suffix[i + 1];
        }

        Self::stone_game_ii_helper(&suffix, 0, 1, &mut mem)
    }

    fn stone_game_ii_helper(suffix: &Vec<i32>, i: usize, m: usize, mem: &mut Vec<Vec<i32>>) -> i32 {
        if i + 2 * m >= mem.len() {
            return suffix[i];
        }
        if mem[i][m] > 0 {
            return mem[i][m];
        }

        let mut opponent = suffix[i];

        for x in 1..=2 * m {
            opponent = min(
                opponent,
                Self::stone_game_ii_helper(suffix, i + x, max(m, x), mem),
            );
        }

        mem[i][m] = suffix[i] - opponent;
        mem[i][m]
    }
}
