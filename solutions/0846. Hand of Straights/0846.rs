// Time complexity: O(sort)
// Space complexity: O(n)
use std::collections::HashMap;

impl Solution {
    pub fn is_n_straight_hand(mut hand: Vec<i32>, group_size: i32) -> bool {
        if hand.len() as i32 % group_size != 0 {
            return false;
        }

        let mut card_count = hand.iter().fold(HashMap::new(), |mut acc, &card| {
            *acc.entry(card).or_insert(0) += 1;
            acc
        });

        hand.sort_unstable();

        for &card in &hand {
            if let Some(&count) = card_count.get(&card) {
                if count == 0 {
                    continue;
                }

                for i in 0..group_size {
                    let next_card = card + i;
                    if let Some(next_count) = card_count.get_mut(&next_card) {
                        if *next_count > 0 {
                            *next_count -= 1;
                        } else {
                            return false;
                        }
                    } else {
                        return false;
                    }
                }
            }
        }

        true
    }
}
