// Time complexity: O(n)
// Space complexity: O(1)
impl Solution {
    pub fn max_distance(arrays: Vec<Vec<i32>>) -> i32 {
        let mut ans = 0;
        let mut mn = 10000;
        let mut mx = -10000;

        for arr in &arrays {
            if let (Some(&first), Some(&last)) = (arr.first(), arr.last()) {
                ans = ans.max(last - mn).max(mx - first);
                mn = mn.min(first);
                mx = mx.max(last);
            }
        }

        ans
    }
}
