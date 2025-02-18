// Time complexity: O(n)
// Space complexity: O(1)
impl Solution {
    pub fn min_operations(logs: Vec<String>) -> i32 {
        logs.iter().fold(0, |acc, log| match log.as_str() {
            "./" => acc,
            "../" => 0.max(acc - 1),
            _ => acc + 1,
        })
    }
}
