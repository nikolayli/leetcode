// Time complexity: O(mn^2)
// Space complexity: O(m)
use std::collections::HashMap;

impl Solution {
    pub fn find_rotate_steps(ring: String, key: String) -> i32 {
        let mut memo: HashMap<String, i32> = HashMap::new();

        Self::dfs(&ring, &key, 0, &mut memo) + key.len() as i32
    }

    fn dfs(ring: &String, key: &String, index: usize, memo: &mut HashMap<String, i32>) -> i32 {
        if index == key.len() {
            return 0;
        }

        let hash_key = format!("{}{}", ring, index);
        if let Some(&res) = memo.get(&hash_key) {
            return res;
        }

        let mut ans = i32::MAX;

        for (i, c) in ring.chars().enumerate() {
            if c == key.chars().nth(index).unwrap() {
                let min_rotates = std::cmp::min(i, ring.len() - i);
                let new_ring = format!("{}{}", &ring[i..], &ring[..i]);
                let remaining_rotates = Self::dfs(&new_ring, &key, index + 1, memo);
                ans = std::cmp::min(ans, (min_rotates + remaining_rotates as usize) as i32);
            }
        }
        memo.insert(hash_key, ans);

        ans
    }
}
