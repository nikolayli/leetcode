// Time complexity: O(n)
// Space complexity: O(n)
impl Solution {
    pub fn interpret(command: String) -> String {
        command.replace("()", "o").replace("(al)", "al")
    }
}
