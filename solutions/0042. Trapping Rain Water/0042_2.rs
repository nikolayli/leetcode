// Time complexity: O(n)
// Space complexity: O(1)
impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        if height.is_empty() {
            return 0;
        }

        let mut ans = 0;
        let mut l = 0;
        let mut r = height.len() - 1;
        let mut max_l = height[l];
        let mut max_r = height[r];

        while l < r {
            if max_l < max_r {
                ans += max_l - height[l];
                max_l = max_l.max(height[l + 1]);
                l += 1;
            } else {
                ans += max_r - height[r];
                max_r = max_r.max(height[r - 1]);
                r -= 1;
            }
        }

        ans
    }
}
