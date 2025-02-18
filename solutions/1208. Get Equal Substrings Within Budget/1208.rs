// Time complexity: O(n)
// Space complexity: O(1)
impl Solution {
    pub fn equal_substring(s: String, t: String, mut max_cost: i32) -> i32 {
        let s = s.as_bytes();
        let t = t.as_bytes();
        let mut j = 0;

        for i in 0..s.len() {
            max_cost -= (s[i] as i32 - t[i] as i32).abs();
            if max_cost < 0 {
                max_cost += (s[j] as i32 - t[j] as i32).abs();
                j += 1;
            }
        }

        (s.len() - j) as i32
    }
}
