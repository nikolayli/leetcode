// Time complexity: O(sqrt(c))
// Space complexity: O(1)
impl Solution {
    pub fn judge_square_sum(c: i32) -> bool {
        let mut l: u32 = 0;
        let mut r: u32 = (c as f64).sqrt() as u32;

        while l <= r {
            let sum = l * l + r * r;
            if sum == c as u32 {
                return true;
            }
            if sum < c as u32 {
                l += 1;
            } else {
                r -= 1;
            }
        }

        false
    }
}
