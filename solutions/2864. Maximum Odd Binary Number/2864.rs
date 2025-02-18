// Time complexity: O(n)
// Space complexity: O(1)
impl Solution {
    pub fn maximum_odd_binary_number(s: String) -> String {
        let zeros = s.chars().filter(|&c| c == '0').count();
        let ones = s.len() - zeros;
        "1".repeat(ones - 1) + &"0".repeat(zeros) + "1"
    }
}
