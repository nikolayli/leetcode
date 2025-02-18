// Time complexity: O(n)
// Space complexity: O(1)
impl Solution {
    pub fn minimum_steps(s: String) -> i64 {
        s.bytes()
            .enumerate()
            .filter(|&(_, c)| c != b'1')
            .enumerate()
            .map(|(j, (i, _))| i as i64 - j as i64)
            .sum()
    }
}
