// Time complexity: O(logn)
// Space complexity: O(1)
impl Solution {
    pub fn sum_of_the_digits_of_harshad_number(x: i32) -> i32 {
        let mut sum = 0;
        let mut n = x;

        while n > 0 {
            sum += n % 10;
            n /= 10;
        }

        if x % sum == 0 {
            sum
        } else {
            -1
        }
    }
}
