// Time complexity: O(n)
// Space complexity: O(1)
impl Solution {
    pub fn min_add_to_make_valid(s: String) -> i32 {
        let mut l = 0;
        let mut r = 0;

        for c in s.chars() {
            if c == '(' {
                l += 1;
            } else {
                if l == 0 {
                    r += 1;
                } else {
                    l -= 1;
                }
            }
        }

        l + r
    }
}
