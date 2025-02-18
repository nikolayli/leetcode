// Time complexity: O(1)
// Space complexity: O(1)
impl Solution {
    pub fn min_end(n: i32, x: i32) -> i64 {
        let mut x = x as i64;
        let mut n = (n - 1) as i64;
        let mut temp = 1;
        for _ in 0..64 {
            if temp & x == 0 {
                x |= (n & 1) * temp;
                n >>= 1;
            }
            temp <<= 1;
        }

        x
    }
}
