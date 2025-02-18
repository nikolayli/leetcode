// Time complexity: O(n)
// Space complexity: O(n)
impl Solution {
    pub fn make_good(s: String) -> String {
        let mut ans = String::new();
        for c in s.chars() {
            if !ans.is_empty() && Self::is_bad_pair(ans.chars().last().unwrap(), c) {
                ans.pop();
            } else {
                ans.push(c);
            }
        }

        ans
    }

    fn is_bad_pair(a: char, b: char) -> bool {
        a != b && a.to_lowercase().to_string() == b.to_lowercase().to_string()
    }
}
