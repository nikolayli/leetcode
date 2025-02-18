// Time complexity: O(n)
// Space complexity: O(1)
impl Solution {
    fn longest_ideal_string(s: String, k: i32) -> i32 {
        let k = k as usize;
        let mut dp = vec![0; 26];

        for c in s.chars() {
            let i = (c as u8 - b'a') as usize;
            dp[i] = 1 + Self::get_max_reachable(&dp, i, k);
        }

        *dp.iter().max().unwrap()
    }

    fn get_max_reachable(dp: &[i32], i: usize, k: usize) -> i32 {
        let first = i32::max(0, (i - k) as i32);
        let last = i32::min(25, (i + k) as i32);
        let mut max_reachable = 0;

        for j in first..=last {
            max_reachable = max_reachable.max(dp[j as usize]);
        }

        max_reachable
    }
}
