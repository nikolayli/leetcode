// Time complexity: O(n^2)
// Space complexity: O(n)
impl Solution {
    pub fn minimum_substrings_in_partition(s: String) -> i32 {
        let n = s.len();
        let mut dp = vec![n as i32; n];
        let s = s.as_bytes();

        for i in 0..n {
            let mut count = vec![0; 26];
            for j in (0..=i).rev() {
                count[(s[j] - b'a') as usize] += 1;
                if Self::is_balanced(&count) {
                    dp[i] = if j > 0 { dp[i].min(1 + dp[j - 1]) } else { 1 };
                }
            }
        }

        *dp.last().unwrap()
    }

    fn is_balanced(count: &Vec<i32>) -> bool {
        let mut min_freq = i32::MAX;
        let mut max_freq = 0;
        for &freq in count {
            if freq > 0 {
                min_freq = min_freq.min(freq);
                max_freq = max_freq.max(freq);
            }
        }

        min_freq == max_freq
    }
}
