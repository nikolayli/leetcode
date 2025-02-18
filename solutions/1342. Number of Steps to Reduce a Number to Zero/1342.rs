// Time complexity: O(1)
// Space complexity: O(1)
impl Solution {
    pub fn number_of_steps(num: i32) -> i32 {
        if num == 0 {
            return 0;
        }
        let subtract_steps = num.count_ones();
        let divide_steps = 31 - num.leading_zeros();
        (subtract_steps + divide_steps) as i32
    }
}
