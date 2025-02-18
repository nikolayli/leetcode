// Time complexity: O(n)
// Space complexity: O(1)
impl Solution {
    pub fn find_different_binary_string(nums: Vec<String>) -> String {
        let mut ans = String::new();

        for (i, num) in nums.iter().enumerate() {
            let bit = if num.chars().nth(i).unwrap() == '0' {
                '1'
            } else {
                '0'
            };
            ans.push(bit)
        }

        ans
    }
}
