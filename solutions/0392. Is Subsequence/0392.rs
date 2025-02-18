// Time complexity: O(n)
// Space complexity: O(1)
impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        if s.is_empty() {
            return true;
        }

        let mut i = 0;
        for c in t.chars() {
            if s.chars().nth(i).unwrap() == c {
                i += 1;
                if i == s.len() {
                    return true;
                }
            }
        }

        false
    }
}
