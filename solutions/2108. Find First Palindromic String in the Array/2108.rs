// Time complexity: O(nm)
// Space complexity: O(1)
impl Solution {
    pub fn first_palindrome(words: Vec<String>) -> String {
        fn is_palindrome(s: &str) -> bool {
            let s = s.as_bytes();
            let mut i = 0;
            let mut j = s.len() - 1;
            while i < j {
                if s[i] != s[j] {
                    return false;
                }
                i += 1;
                j -= 1;
            }
            true
        }

        for word in words {
            if is_palindrome(&word) {
                return word;
            }
        }
        String::new()
    }
}
