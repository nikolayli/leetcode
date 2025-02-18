// Time complexity: O(sort)
// Space complexity: O(n)
impl Solution {
    pub fn number_game(mut nums: Vec<i32>) -> Vec<i32> {
        nums.sort();

        for i in (0..nums.len()).step_by(2) {
            if i + 1 < nums.len() {
                nums.swap(i, i + 1);
            }
        }

        nums
    }
}
