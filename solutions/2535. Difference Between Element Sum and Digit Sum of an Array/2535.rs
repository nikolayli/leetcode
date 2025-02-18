// Time complexity: O(nm)
// Space complexity: O(m)
impl Solution {
    pub fn difference_of_sum(nums: Vec<i32>) -> i32 {
        let element_sum: i32 = nums.iter().sum();
        let digit_sum: i32 = Self::get_all_digit_sum(&nums);
        (element_sum - digit_sum).abs()
    }

    fn get_all_digit_sum(nums: &[i32]) -> i32 {
        nums.iter().map(|&num| Self::get_digit_sum(num)).sum()
    }

    fn get_digit_sum(mut num: i32) -> i32 {
        let mut sum = 0;
        while num > 0 {
            sum += num % 10;
            num /= 10;
        }
        sum
    }
}
