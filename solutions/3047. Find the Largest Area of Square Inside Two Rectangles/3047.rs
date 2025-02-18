// Time complexity: O(n^2)
// Space complexity: O(1)
impl Solution {
    pub fn largest_square_area(bottom_left: Vec<Vec<i32>>, top_right: Vec<Vec<i32>>) -> i64 {
        let mut max_area: i64 = 0;

        for i in 0..bottom_left.len() {
            for j in (i + 1)..bottom_left.len() {
                let x1: i64 = bottom_left[i][0].max(bottom_left[j][0]).into();
                let y1: i64 = bottom_left[i][1].max(bottom_left[j][1]).into();
                let x2: i64 = top_right[i][0].min(top_right[j][0]).into();
                let y2: i64 = top_right[i][1].min(top_right[j][1]).into();

                if x1 < x2 && y1 < y2 {
                    let side: i64 = (x2 - x1).min(y2 - y1);
                    max_area = max_area.max(side * side);
                }
            }
        }

        max_area
    }
}
