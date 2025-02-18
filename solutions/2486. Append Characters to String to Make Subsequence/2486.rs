// Time complexity: O(n)
// Space complexity: O(1)
impl Solution {
    pub fn append_characters(s: String, t: String) -> i32 {
        let mut i = 0;
        let t: Vec<char> = t.chars().collect();

        for c in s.chars() {
            if i < t.len() && c == t[i] {
                i += 1;
                if i == t.len() {
                    return 0;
                }
            }
        }

        (t.len() - i) as i32
    }
}
