// Time complexity: O(n)
// Space complexity: O(1)
impl Solution {
    pub fn check_straight_line(coordinates: Vec<Vec<i32>>) -> bool {
        let (x0, y0) = (coordinates[0][0], coordinates[0][1]);
        let (x1, y1) = (coordinates[1][0], coordinates[1][1]);
        let dx = x1 - x0;
        let dy = y1 - y0;

        for i in 2..coordinates.len() {
            let (x, y) = (coordinates[i][0], coordinates[i][1]);
            if (x - x0) * dy != (y - y0) * dx {
                return false;
            }
        }

        true
    }
}
