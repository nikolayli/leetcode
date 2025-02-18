// Time complexity: O(n)
// Space complexity: O(1)
impl Solution {
    pub fn minimum_chairs(s: String) -> i32 {
        let mut ans = 0;
        let mut count = 0;

        for c in s.chars() {
            if c == 'E' {
                count += 1;
            } else {
                count -= 1;
            }
            ans = ans.max(count);
        }

        ans
    }
}
