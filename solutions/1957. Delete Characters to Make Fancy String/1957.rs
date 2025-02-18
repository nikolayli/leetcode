// Time complexity: O(n)
// Space complexity: O(n)
impl Solution {
    pub fn make_fancy_string(s: String) -> String {
        let mut ans = String::new();

        for c in s.chars() {
            if ans.len() < 2
                || ans.as_bytes()[ans.len() - 1] != c as u8
                || ans.as_bytes()[ans.len() - 2] != c as u8
            {
                ans.push(c);
            }
        }

        ans
    }
}
