// Time complexity: O(n + m)
// Space complexity: O(n)
impl Solution {
    pub fn count_matching_subarrays(nums: Vec<i32>, pattern: Vec<i32>) -> i32 {
        let mut nums_pattern = Vec::new();
        for i in 1..nums.len() {
            nums_pattern.push((nums[i] - nums[i - 1]).signum())
        }
        Self::rabin_karp(&nums_pattern, &pattern)
    }

    fn rabin_karp(nums_pattern: &Vec<i32>, pattern: &Vec<i32>) -> i32 {
        let p: i64 = 31;
        let k_mod: i64 = 1_000_000_001;
        let mut pattern_pow: i64 = 1;
        let mut h_t: i64 = 0;
        let mut h_s: i64 = 0;
        let mut res: i32 = 0;

        for i in 0..pattern.len() {
            h_t = (h_t * p + (pattern[i] as i64 + 1)) % k_mod;
            pattern_pow = (pattern_pow * p) % k_mod;
        }

        for i in 0..nums_pattern.len() {
            h_s = (h_s * p + (nums_pattern[i] as i64 + 1)) % k_mod;
            if i >= pattern.len() {
                h_s = (k_mod + h_s
                    - pattern_pow * (nums_pattern[i - pattern.len()] as i64 + 1) % k_mod)
                    % k_mod;
            }
            if i + 1 >= pattern.len() as usize && h_t == h_s {
                res += 1;
            }
        }

        res
    }
}
