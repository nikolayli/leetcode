// Time complexity: O(n)
// Space complexity: O(n)
impl Solution {
    pub fn shortest_palindrome(s: String) -> String {
        let hash_base: i64 = 29;
        let k_mod = 1_000_000_007;
        let mut forw_hash: i64 = 0;
        let mut rev_hash: i64 = 0;
        let mut pow_val: i64 = 1;
        let mut index: i32 = -1;

        for (i, curr_char) in s.chars().enumerate() {
            let char_val = (curr_char as i64) - ('a' as i64) + 1;
            forw_hash = (forw_hash * hash_base + char_val) % k_mod;
            rev_hash = (rev_hash + char_val * pow_val) % k_mod;
            pow_val = (pow_val * hash_base) % k_mod;

            if forw_hash == rev_hash {
                index = i as i32;
            }
        }

        let suffix = &s[(index as usize + 1)..];
        let rev_suffix: String = suffix.chars().rev().collect();

        rev_suffix + &s
    }
}
