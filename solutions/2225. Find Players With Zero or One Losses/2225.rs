// Time complexity: O(nlogn)
// Space complexity: O(n)
use std::collections::HashMap;

impl Solution {
    pub fn find_winners(matches: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut ans: Vec<Vec<i32>> = vec![vec![], vec![]];
        let mut losses_count: HashMap<i32, i32> = HashMap::new();

        for m in matches {
            let winner = m[0];
            let loser = m[1];
            *losses_count.entry(winner).or_insert(0) += 0;
            *losses_count.entry(loser).or_insert(0) += 1;
        }

        for (player, n_losses) in &losses_count {
            if *n_losses < 2 {
                ans[*n_losses as usize].push(*player);
            }
        }

        ans.iter_mut().for_each(|v| v.sort_unstable());

        ans
    }
}
