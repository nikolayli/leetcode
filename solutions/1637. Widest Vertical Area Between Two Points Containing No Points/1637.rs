// Time complexity: O(sort)
// Space complexity: O(n)
impl Solution {
    pub fn max_width_of_vertical_area(mut points: Vec<Vec<i32>>) -> i32 {
        points.sort_unstable_by_key(|v| v[0]);
        points.windows(2).map(|v| v[1][0] - v[0][0]).max().unwrap()
    }
}
