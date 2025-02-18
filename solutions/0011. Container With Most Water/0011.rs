// Time complexity: O(n)
// Space complexity: O(1)
impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut ans = 0;
        let mut l = 0;
        let mut r = height.len() - 1;

        while l < r {
            let min_height = height[l].min(height[r]);
            ans = ans.max(min_height * (r - l) as i32);
            if height[l] < height[r] {
                l += 1;
            } else {
                r -= 1;
            }
        }

        ans
    }
}
