// Time complexity: O(logn)
// Space complexity: O(1)
pub struct Solution;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        }

        let mut y = x;
        let mut reversed: i32 = 0;

        while y > 0 {
            reversed = reversed * 10 + (y % 10);
            y /= 10;
        }

        reversed == x
    }
}
