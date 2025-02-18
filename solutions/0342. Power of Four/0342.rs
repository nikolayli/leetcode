// Time complexity: O(logn)
// Space complexity: O(1)
impl Solution {
    pub fn is_power_of_four(mut n: i32) -> bool {
        if n <= 0 {
            return false;
        }

        while n % 4 == 0 {
            n /= 4;
        }

        return n == 1;
    }
}
