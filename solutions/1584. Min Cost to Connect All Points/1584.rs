// Time complexity: O(n^2)
// Space complexity: O(n)
impl Solution {
    pub fn min_cost_connect_points(mut points: Vec<Vec<i32>>) -> i32 {
        let mut dist: Vec<i32> = vec![i32::MAX; points.len()];
        let mut ans = 0;

        for i in 0..points.len() - 1 {
            for j in i + 1..points.len() {
                let manhattan_distance =
                    (points[i][0] - points[j][0]).abs() + (points[i][1] - points[j][1]).abs();
                dist[j] = dist[j].min(manhattan_distance);

                if dist[j] < dist[i + 1] {
                    dist.swap(j, i + 1);
                    points.swap(j, i + 1);
                }
            }
            ans += dist[i + 1];
        }

        ans
    }
}
