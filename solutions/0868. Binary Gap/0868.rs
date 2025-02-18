// Time Complexity: O(log⁡n)
// Space Complexity: O(1)
impl Solution {
    pub fn binary_gap(n: i32) -> i32 {
        let mut ans = 0;
        let mut d = -32;

        let mut num = n;
        while num != 0 {
            if num % 2 == 1 {
                ans = ans.max(d);
                d = 0;
            }
            num /= 2;
            d += 1;
        }

        ans
    }
}
