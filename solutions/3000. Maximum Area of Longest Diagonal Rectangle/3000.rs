// Time complexity: O(n)
// Space complexity: O(1)
impl Solution {
    pub fn area_of_max_diagonal(dimensions: Vec<Vec<i32>>) -> i32 {
        dimensions
            .iter()
            .map(|v| (v[0] * v[0] + v[1] * v[1], v[0] * v[1]))
            .max()
            .unwrap()
            .1
    }
}
