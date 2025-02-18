// Time complexity: O(d)
// Space complexity: O(1)
impl Solution {
    pub fn count_digits(num: i32) -> i32 {
        let mut ans = 0;
        let mut n = num;

        while n > 0 {
            if num % (n % 10) == 0 {
                ans += 1;
            }
            n /= 10;
        }

        ans
    }
}
