// Time complexity: O(mn)
// Space complexity: O(m)
impl Solution {
    pub fn count_points(points: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        queries
            .iter()
            .map(|query| {
                let (xj, yj, rj) = (query[0], query[1], query[2]);
                points
                    .iter()
                    .filter(|point| {
                        let (xi, yi) = (point[0], point[1]);
                        (xi - xj).pow(2) + (yi - yj).pow(2) <= rj.pow(2)
                    })
                    .count() as i32
            })
            .collect()
    }
}
