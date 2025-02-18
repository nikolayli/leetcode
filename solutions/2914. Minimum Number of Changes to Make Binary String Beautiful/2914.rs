// Time complexity: O(n)
// Space complexity: O(1)
impl Solution {
    pub fn min_changes(s: String) -> i32 {
        s.as_bytes().chunks(2).filter(|c| c[0] != c[1]).count() as i32
    }
}
