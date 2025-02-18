// Time complexity: O(sort)
// Space complexity: O(n)
impl Solution {
    pub fn height_checker(heights: Vec<i32>) -> i32 {
        let mut expected = heights.clone();
        expected.sort_unstable();
        let mut ans = 0;

        for (h, e) in heights.iter().zip(expected.iter()) {
            if h != e {
                ans += 1;
            }
        }

        ans
    }
}
