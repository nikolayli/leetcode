// Time complexity: O(n)
// Space complexity: O(1)
impl Solution {
    pub fn count_homogenous(s: String) -> i32 {
        const K_MOD: i32 = 1_000_000_007;
        let mut ans = 0;
        let mut count = 0;
        let mut current_char = '@';

        for c in s.chars() {
            count = if c == current_char { count + 1 } else { 1 };
            current_char = c;
            ans += count;
            ans %= K_MOD;
        }

        ans
    }
}
