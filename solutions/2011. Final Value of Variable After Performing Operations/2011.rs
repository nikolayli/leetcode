// Time complexity: O(n)
// Space complexity: O(1)
impl Solution {
    pub fn final_value_after_operations(operations: Vec<String>) -> i32 {
        operations
            .iter()
            .map(|op| if op.as_bytes()[1] == b'+' { 1 } else { -1 })
            .sum()
    }
}
