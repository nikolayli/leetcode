// Time complexity: O(n)
// Space complexity: O(n)
impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let n = height.len();
        let mut ans = 0;
        let mut l = vec![0; n];
        let mut r = vec![0; n];

        for i in 0..n {
            l[i] = if i == 0 {
                height[i]
            } else {
                height[i].max(l[i - 1])
            };
        }

        for i in (0..n).rev() {
            r[i] = if i == n - 1 {
                height[i]
            } else {
                height[i].max(r[i + 1])
            };
        }

        for i in 0..n {
            ans += l[i].min(r[i]) - height[i];
        }

        ans
    }
}
