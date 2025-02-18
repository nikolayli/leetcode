// Time complexity: O(mn)
// Space complexity: O(mn)
impl Solution {
    pub fn construct2_d_array(original: Vec<i32>, m: i32, n: i32) -> Vec<Vec<i32>> {
        let n = n as usize;
        let m = m as usize;

        if original.len() != n * m {
            return vec![];
        }

        original.windows(n).step_by(n).map(|v| v.to_vec()).collect()
    }
}
