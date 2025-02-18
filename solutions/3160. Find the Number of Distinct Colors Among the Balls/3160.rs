// Time complexity: O(n)
// Space complexity: O(n)
use std::collections::HashMap;

impl Solution {
    pub fn query_results(limit: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut ball_to_color: HashMap<i32, i32> = HashMap::new();
        let mut color_count: HashMap<i32, i32> = HashMap::new();
        let mut ans: Vec<i32> = Vec::new();

        for query in queries.iter() {
            let ball = query[0];
            let color = query[1];
            if let Some(&prev_color) = ball_to_color.get(&ball) {
                let count = color_count.entry(prev_color).or_insert(0);
                *count -= 1;
                if *count == 0 {
                    color_count.remove(&prev_color);
                }
            }
            ball_to_color.insert(ball, color);
            *color_count.entry(color).or_insert(0) += 1;
            ans.push(color_count.len() as i32);
        }

        ans
    }
}
