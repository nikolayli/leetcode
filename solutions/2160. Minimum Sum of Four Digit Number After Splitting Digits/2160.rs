// Time complexity: O(1)
// Space complexity: O(1)
impl Solution {
    pub fn minimum_sum(num: i32) -> i32 {
        let mut digits: Vec<char> = num.to_string().chars().collect();
        digits.sort_unstable();

        let num1: i32 = (digits[0].to_digit(10).unwrap() as i32) * 10
            + (digits[2].to_digit(10).unwrap() as i32);
        let num2: i32 = (digits[1].to_digit(10).unwrap() as i32) * 10
            + (digits[3].to_digit(10).unwrap() as i32);

        num1 + num2
    }
}
