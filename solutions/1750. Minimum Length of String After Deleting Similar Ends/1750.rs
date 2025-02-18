// Time complexity: O(n)
// Space complexity: O(1)
impl Solution {
    pub fn minimum_length(s: String) -> i32 {
        let mut s = s.as_str();
        while s.len() > 1 && s.ends_with(unsafe { s.get_unchecked(0..1) }) {
            s = s.trim_matches(s.chars().next().unwrap())
        }

        s.len() as _
    }
}
