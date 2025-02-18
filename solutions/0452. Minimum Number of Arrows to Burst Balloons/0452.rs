// Time complexity: O(nlogn)
// Space complexity: O(1)
impl Solution {
    pub fn find_min_arrow_shots(mut points: Vec<Vec<i32>>) -> i32 {
        points.sort_unstable_by_key(|b| b[1]);
        let mut ans = 1;
        let mut arrow_x = points[0][1];

        for i in 1..points.len() {
            if points[i][0] > arrow_x {
                arrow_x = points[i][1];
                ans += 1;
            }
        }

        ans
    }
}
