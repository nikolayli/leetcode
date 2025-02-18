// Time complexity: O(n)
// Space complexity: O(1)
impl Solution {
    pub fn min_steps(n: i32) -> i32 {
        let mut ans = 0;
        let mut d = 2;

        while n > 1 {
            while n % d == 0 {
                ans += d;
                n /= d;
            }
            d += 1;
        }

        ans
    }
}
