// Time complexity: O(sort)
// Space complexity: O(1)
impl Solution {
    pub fn min_rectangles_to_cover_points(mut points: Vec<Vec<i32>>, w: i32) -> i32 {
        points.sort_unstable_by(|a, b| a.cmp(b));

        let mut ans = 0;
        let mut idx = -1;

        for point in points.iter() {
            if point[0] > idx {
                ans += 1;
                idx = point[0] + w;
            }
        }

        ans
    }
}
