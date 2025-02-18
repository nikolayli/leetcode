// Time complexity: O(n)
// Space complexity: O(1)
impl Solution {
    pub fn find_permutation_difference(s: String, t: String) -> i32 {
        let mut ans = 0;
        let mut indices = [0; 26];

        for (i, ch) in t.chars().enumerate() {
            indices[(ch as usize) - ('a' as usize)] = i;
        }

        for (i, ch) in s.chars().enumerate() {
            ans += (i as i32 - indices[(ch as usize) - ('a' as usize)] as i32).abs();
        }

        ans
    }
}
