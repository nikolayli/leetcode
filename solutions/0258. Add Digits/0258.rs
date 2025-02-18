// Time Complexity: O(1)
// Space Complexity: O(1)
impl Solution {
    pub fn add_digits(num: i32) -> i32 {
        return (num - 1) % 9 + 1;
    }
}
