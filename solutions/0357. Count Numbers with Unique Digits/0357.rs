// Time complexity: O(n)
// Space complexity: O(1)
impl Solution {
    pub fn count_numbers_with_unique_digits(n: i32) -> i32 {
        if n == 0 {
            return 1;
        }

        let mut ans = 10;
        let mut unique_digits = 9;
        let mut available_num = 9;

        for _ in 1..n {
            unique_digits *= available_num;
            ans += unique_digits;
            available_num -= 1;
        }

        ans
    }
}
