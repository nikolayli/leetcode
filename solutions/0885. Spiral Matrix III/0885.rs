// Time complexity: O(max(R,C)^2)
// Space complexity: O(RC)
impl Solution {
    pub fn spiral_matrix_iii(
        rows: i32,
        cols: i32,
        mut r_start: i32,
        mut c_start: i32,
    ) -> Vec<Vec<i32>> {
        let dx = [1, 0, -1, 0];
        let dy = [0, 1, 0, -1];
        let mut ans = vec![vec![r_start, c_start]];
        let mut i = 0;

        while ans.len() < (rows * cols) as usize {
            for _ in 0..(i / 2 + 1) {
                r_start += dy[(i % 4) as usize];
                c_start += dx[(i % 4) as usize];
                if r_start >= 0 && r_start < rows && c_start >= 0 && c_start < cols {
                    ans.push(vec![r_start, c_start]);
                }
            }
            i += 1;
        }

        ans
    }
}
