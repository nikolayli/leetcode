// Time complexity: O(1)
// Space complexity: O(1)
impl Solution {
    pub fn is_power_of_three(n: i32) -> bool {
        n > 0 && 3_i32.pow(19) % n == 0;
    }
}
