// Time complexity: O(n)
// Space complexity: O(n)
impl Solution {
    pub fn valid_palindrome(s: String) -> bool {
        let s = s.as_bytes();
        let n = s.len();
        for i in 0..n / 2 {
            if s[i] != s[n - i - 1] {
                let l = &s[i..n - i - 1];
                let r = &s[i + 1..n - i];
                return Self::helper(l) || Self::helper(r);
            }
        }
        true
    }
    fn helper(s: &[u8]) -> bool {
        let n = s.len();
        (0..n / 2).all(|i| s[i] == s[n - i - 1])
    }
}
