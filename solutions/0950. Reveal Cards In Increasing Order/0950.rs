// Time complexity: O(sort)
// Space complexity: O(n)
use std::collections::VecDeque;

impl Solution {
    pub fn deck_revealed_increasing(mut deck: Vec<i32>) -> Vec<i32> {
        deck.sort_unstable_by(|a, b| b.cmp(a));

        let mut dq: VecDeque<i32> = VecDeque::new();
        dq.push_back(deck[0]);

        for i in 1..deck.len() {
            let last = dq.pop_back().unwrap();
            dq.push_front(last);
            dq.push_front(deck[i]);
        }

        dq.into_iter().collect()
    }
}
