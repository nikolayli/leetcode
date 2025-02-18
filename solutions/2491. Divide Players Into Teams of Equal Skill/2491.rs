// Time complexity: O(n)
// Space complexity: O(n)
use std::collections::HashMap;

impl Solution {
    pub fn divide_players(skill: Vec<i32>) -> i64 {
        let n = skill.len() as i32;
        let team_skill = skill.iter().sum::<i32>() / (n / 2);
        let mut count: HashMap<i32, i32> = HashMap::new();
        let mut ans = 0;

        for &s in &skill {
            *count.entry(s).or_insert(0) += 1;
        }

        for (&s, &freq) in &count {
            let required_skill = team_skill - s;
            if let Some(&req_freq) = count.get(&required_skill) {
                if req_freq != freq {
                    return -1;
                }
                ans += s as i64 * required_skill as i64 * freq as i64;
            } else {
                return -1;
            }
        }

        ans / 2
    }
}
