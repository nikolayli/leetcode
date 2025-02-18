// Time complexity: O(2^n)
// Space complexity: O(n)
impl Solution {
    pub fn max_length(arr: Vec<String>) -> i32 {
        fn get_mask(s: &str) -> Option<i32> {
            let mut mask = 0;
            for c in s.chars() {
                let i = (c as u8 - b'a') as i32;
                if (mask & (1 << i)) != 0 {
                    return None;
                }
                mask |= 1 << i;
            }
            Some(mask)
        }

        fn dfs(masks: &[i32], s: usize, used: i32) -> i32 {
            let mut res = used.count_ones() as i32;
            for i in s..masks.len() {
                if (used & masks[i]) == 0 {
                    res = res.max(dfs(masks, i + 1, used | masks[i]));
                }
            }

            res
        }

        let masks: Vec<i32> = arr.iter().filter_map(|s| get_mask(s)).collect();

        dfs(&masks, 0, 0)
    }
}
