// Time complexity: O(n)
// Space complexity: O(1)
impl Solution {
    pub fn max_depth(s: String) -> i32 {
        s.bytes()
            .filter_map(|byte| match byte {
                b'(' => Some(1),
                b')' => Some(-1),
                _ => None,
            })
            .scan(0, |ans, opened| {
                *ans += opened;
                Some(*ans)
            })
            .max()
            .unwrap_or(0)
    }
}
